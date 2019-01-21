use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use parking_lot::RwLock;
use serde::de::DeserializeOwned;

use super::register::{RegisterInfo, RegisterList, RegisterRecord};
use crate::{BlockQueue, Result};

const MSG_CHUNK_SIZE_LIMIT: usize = 10;

#[derive(PartialEq, Clone, Debug, Serialize)]
pub struct Message {
    height: u64,
    data: Vec<serde_json::Value>,
}

impl Message {
    pub fn new(height: u64) -> Self {
        Self {
            height,
            data: vec![],
        }
    }

    pub fn add(&mut self, value: serde_json::Value) {
        self.data.push(value);
    }

    /// Split the message into multiple messages according to `chunk_size`.
    pub fn split(self, chunk_size: usize) -> Vec<Self> {
        debug!("The message was split into multiple messages");
        let chunks = self
            .data
            .chunks(chunk_size)
            .map(|value| value.to_vec())
            .collect::<Vec<Vec<serde_json::Value>>>();
        chunks
            .into_iter()
            .map(|chunk| Message {
                height: self.height,
                data: chunk,
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    retry_count: u32,
    retry_interval: Duration,
}

impl Config {
    pub fn new(retry_count: u32, retry_interval: Duration) -> Self {
        Self {
            retry_count,
            retry_interval,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(3, Duration::new(3, 0))
    }
}

type PushFlag = Arc<RwLock<HashMap<String, bool>>>;

#[derive(Debug)]
pub struct PushService {
    register_list: RegisterList,
    block_queue: BlockQueue,
    config: Config,
    push_flag: PushFlag,
    client: reqwest::Client,
}

impl PushService {
    pub fn new(register_list: RegisterList, block_queue: BlockQueue, config: Config) -> Self {
        Self {
            register_list,
            block_queue,
            config,
            push_flag: Default::default(),
            client: reqwest::Client::new(),
        }
    }

    pub fn start(&mut self) {
        loop {
            if self.block_queue.read().is_empty() {
                continue;
            };
            let cur_block_height = self.get_block_height();
            let (tx, rx) = mpsc::channel();
            let mut have_new_push = false;
            for (url, info) in self.register_list.read().unwrap().iter() {
                let push_height = info.lock().unwrap().status.height;
                let is_down = info.lock().unwrap().status.down;
                if cur_block_height >= push_height && !is_down {
                    self.push_flag
                        .write()
                        .entry(url.clone())
                        .or_insert_with(|| {
                            info!(
                                "have new push! cur_block_height:{:?}, push_height: {:?}",
                                cur_block_height, push_height
                            );
                            have_new_push = true;
                            self.push_message(cur_block_height, url, info.clone(), tx.clone());
                            true
                        });
                }
            }

            if have_new_push {
                self.receive(rx);
            }
        }
    }

    fn push_message(
        &self,
        cur_push_height: u64,
        url: &str,
        reg_data: RegisterInfo,
        tx: Sender<String>,
    ) {
        let queue = self.block_queue.clone();
        let config = self.config;
        let url = url.to_string();
        thread::spawn(move || {
            if let Ok(mut reg) = reg_data.lock() {
                while reg.status.height <= cur_push_height {
                    if let Some(msg) = build_message(&queue, reg.status.height, &reg.prefix) {
                        if send_large_message(&url, msg, &config).is_err() {
                            reg.switch_off();
                            break;
                        }
                        debug!("Send messages ok, url: {}", url);
                    }
                    reg.add_height();
                }
                tx.send(url).unwrap();
            };
        });
    }

    fn receive(&self, rx: Receiver<String>) {
        debug!("receive");
        let list = self.register_list.clone();
        let queue = self.block_queue.clone();
        let push_flag = self.push_flag.clone();
        let cur_block_height = self.get_block_height();
        thread::spawn(move || {
            for url in rx {
                info!("receive url: {:?}", url);
                push_flag.write().remove(&url);
                RegisterRecord::save(&json!(list).to_string()).expect("record save error");
            }
            delete_msg(&list, &queue, cur_block_height);
            debug!("receive end");
        });
    }

    /// Get the max key of BTreeMap, which is current block height.
    fn get_block_height(&self) -> u64 {
        match self.block_queue.read().keys().next_back() {
            Some(s) => *s,
            None => 0,
        }
    }
}

/// Find the values, in the queue, that match the prefixes from the registrant,
/// and construct push message for registrant.
fn build_message(queue: &BlockQueue, height: u64, prefixes: &[String]) -> Option<Message> {
    if let Some(values) = queue.read().get(&height) {
        let mut push_msg = Message::new(height);
        for value in values {
            let msg_prefix: String = serde_json::from_value(value["prefix"].clone()).unwrap();
            for prefix in prefixes {
                debug!("prefix: {:?}, msg_prefix: {:?}", prefix, msg_prefix);
                if *prefix == msg_prefix {
                    debug!("Match prefix");
                    push_msg.add(value.clone());
                }
            }
        }
        if !push_msg.data.is_empty() {
            return Some(push_msg);
        }
    } else {
        warn!("Cannot find info of block height : {:?}", height);
    }
    None
}

fn send_large_message(url: &str, msg: Message, config: &Config) -> Result<()> {
    let messages = msg.split(MSG_CHUNK_SIZE_LIMIT);
    for msg in messages {
        if let Err(err) = send_message(url, msg, config) {
            return Err(err);
        }
    }
    Ok(())
}

fn send_message(url: &str, msg: Message, config: &Config) -> Result<()> {
    let body = json!(msg);
    debug!("Send message request: {:?}", body);
    for i in 1..=config.retry_count {
        let ok = request::<String>(url, &body)?;
        info!("Receive message response: {:?}", ok);
        if ok == "OK" {
            return Ok(());
        }
        warn!(
            "Send message request retry ({} / {})",
            i, config.retry_count
        );
        thread::sleep(config.retry_interval);
    }
    warn!(
        "Reach the limitation of retries, failed to send message: {:?}",
        msg
    );
    Err("Reach the limitation of retries".into())
}

#[derive(Debug, Deserialize)]
struct JsonResponse<T> {
    result: T,
}

fn request<T>(url: &str, body: &serde_json::Value) -> Result<T>
where
    T: Debug + DeserializeOwned,
{
    let resp: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(body)
        .send()?
        .json::<serde_json::Value>()?;
    let resp: JsonResponse<T> = serde_json::from_value(resp)?;
    Ok(resp.result)
}

fn delete_msg(list: &RegisterList, queue: &BlockQueue, cur_block_height: u64) {
    let mut min_push_height = u64::max_value();
    for info in list.read().unwrap().values() {
        let info = info.lock().unwrap();
        if !info.status.down && info.status.height > 0 && info.status.height - 1 < min_push_height {
            min_push_height = info.status.height - 1;
        }
    }

    if min_push_height <= cur_block_height {
        let mut h = *queue.read().keys().next().unwrap();
        info!(
            "cur_block_height: {:?}, min_push_height: {:?}, queue len: {:?}",
            h,
            min_push_height,
            queue.read().len()
        );
        while h <= min_push_height {
            match queue.write().remove(&h) {
                Some(_) => info!("del msg: {:?}", h),
                None => warn!("error: no key: {:?}", h),
            };
            h += 1;
        }
    } else {
        warn!(
            "delete msg error! min_push_height: {:?}, cur_block_height: {:?}",
            min_push_height, cur_block_height
        );
        queue.write().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_split() {
        macro_rules! value {
            ($v:expr) => {
                serde_json::from_str::<serde_json::Value>($v).unwrap()
            };
        }

        let message = Message {
            height: 123,
            data: vec![
                value!("1"),
                value!("2"),
                value!("3"),
                value!("4"),
                value!("5"),
            ],
        };

        assert_eq!(
            vec![
                Message {
                    height: 123,
                    data: vec![value!("1"), value!("2")]
                },
                Message {
                    height: 123,
                    data: vec![value!("3"), value!("4")]
                },
                Message {
                    height: 123,
                    data: vec![value!("5")]
                },
            ],
            message.clone().split(2)
        );

        assert_eq!(vec![message.clone()], message.split(5));
    }
}
