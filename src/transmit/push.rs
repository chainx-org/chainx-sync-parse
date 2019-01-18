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

#[derive(Debug, Deserialize)]
struct JsonResponse<T> {
    result: T,
}

fn request<T: Debug + DeserializeOwned>(url: &str, body: &serde_json::Value) -> Result<T> {
    let resp: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(body)
        .send()?
        .json::<serde_json::Value>()?;
    let resp: JsonResponse<T> = serde_json::from_value(resp)?;
    Ok(resp.result)
}

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
        if self.data.len() > chunk_size {
            let mut messages = vec![];
            let chunks = self
                .data
                .chunks(chunk_size)
                .map(|value| value.to_vec())
                .collect::<Vec<Vec<serde_json::Value>>>();
            for chunk in chunks {
                messages.push(Message {
                    height: self.height,
                    data: chunk,
                });
            }
            messages
        } else {
            vec![self]
        }
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

type PushFlag = Arc<RwLock<HashMap<String, bool>>>;

#[derive(Debug)]
pub struct PushClient {
    register_list: RegisterList,
    block_queue: BlockQueue,
    config: Config,
    push_flag: PushFlag,
    inner: reqwest::Client,
}

impl PushClient {
    pub fn new(register_list: RegisterList, block_queue: BlockQueue, config: Config) -> Self {
        Self {
            register_list,
            block_queue,
            config,
            push_flag: Default::default(),
            inner: reqwest::Client::new(),
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
                            self.push_msg(cur_block_height, url, info.clone(), tx.clone());
                            true
                        });
                }
            }

            if have_new_push {
                self.receive(rx);
            }
        }
    }

    fn request<T>(&self, url: &str, body: &serde_json::Value) -> Result<T>
    where
        T: Debug + DeserializeOwned,
    {
        let resp = self
            .inner
            .post(url)
            .json(body)
            .send()?
            .json::<JsonResponse<T>>()?;
        Ok(resp.result)
    }

    fn push_msg(
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
                    if let Some(msg) = is_post_msg(&queue, reg.status.height, &reg.prefix) {
                        info!("should post!");
                        if !post_msg(&url, msg, &config) {
                            warn!("post err");
                            reg.switch_off();
                            break;
                        }
                        debug!("post ok");
                    }
                    reg.add_height();
                }
                debug!("post end");
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

fn is_post_msg(queue: &BlockQueue, height: u64, prefixes: &[String]) -> Option<Message> {
    if let Some(msg) = queue.read().get(&height) {
        let mut push_msg = Message::new(height);
        for v in msg {
            let msg_prefix: String = serde_json::from_str(&v["prefix"].to_string()).unwrap();
            for prefix in prefixes {
                debug!("prefix: {:?}, msg_prefix: {:?}", *prefix, msg_prefix);
                if *prefix == msg_prefix {
                    debug!("find prefix");
                    push_msg.add(v.clone());
                }
            }
        }
        if !push_msg.data.is_empty() {
            return Some(push_msg);
        }
    } else {
        warn!("can not find msg! height: {:?}", height);
    }
    None
}

fn post_msg(url: &str, msg: Message, config: &Config) -> bool {
    debug!("post");
    let slice_msg = msg.split(10);
    for msg in slice_msg {
        debug!("msg:{:?}", msg);
        let json = json!(msg);
        let mut flag = true;
        for i in 0..config.retry_count {
            if let Ok(ok) = request::<String>(url, &json) {
                info!("post res: {:?}", ok);
                if ok == "OK" {
                    flag = true;
                    break;
                }
            }
            info!("retry count: {:?}", i);
            flag = false;
            thread::sleep(config.retry_interval);
        }
        if !flag {
            return false;
        }
    }
    true
}

fn delete_msg(list: &RegisterList, queue: &BlockQueue, cur_block_height: u64) {
    let mut min_push_height = u64::max_value();
    for register in list.read().unwrap().values() {
        let reg = register.lock().unwrap();
        if !reg.status.down && reg.status.height > 0 && reg.status.height - 1 < min_push_height {
            min_push_height = reg.status.height - 1;
        }
    }

    if min_push_height <= cur_block_height {
        let mut h = *queue.read().keys().next().unwrap();
        info!(
            "cur_block_height: {:?}, min_push_height: {:?},queue len: {:?}",
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
    fn message_split() {
        macro_rules! value {
            ($v:expr) => {{
                serde_json::from_str::<serde_json::Value>($v).unwrap()
            }};
        }

        let message = Message {
            height: 123,
            data: vec![value!("1"), value!("2"), value!("3"), value!("4"), value!("5")],
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

        assert_eq!(
            vec![message.clone()],
            message.split(5)
        );
    }
}
