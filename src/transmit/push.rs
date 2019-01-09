use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use serde::de::DeserializeOwned;

use super::register::{RegisterInfo, RegisterList, RegisterRecord};
use crate::{Arc, BlockQueue, Result, RwLock};

#[derive(Deserialize, Debug)]
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

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    height: u64,
    data: Vec<serde_json::Value>,
}

impl Message {
    pub fn new(num: u64) -> Self {
        Self {
            height: num,
            data: vec![],
        }
    }

    pub fn add(&mut self, date: serde_json::Value) {
        self.data.push(date);
    }
}

#[derive(Debug, Clone)]
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
}

impl PushClient {
    pub fn new(register_list: RegisterList, block_queue: BlockQueue, config: Config) -> Self {
        Self {
            register_list,
            block_queue,
            config,
            push_flag: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn get_block_height(&self) -> u64 {
        match self.block_queue.read().keys().next_back() {
            Some(s) => *s,
            None => 0,
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
                            info!("have new push!");
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

    fn push_msg(
        &self,
        cur_push_height: u64,
        url: &str,
        reg_data: RegisterInfo,
        tx: Sender<String>,
    ) {
        let queue = self.block_queue.clone();
        let config = self.config.clone();
        let url = url.to_string();
        thread::spawn(move || {
            if let Ok(mut reg) = reg_data.lock() {
                info!("cur_push_height: {:?}", cur_push_height);
                while reg.status.height <= cur_push_height {
                    if let Some(msg) =
                        is_post_msg(queue.clone(), reg.status.height, reg.prefix.clone())
                    {
                        if !post_msg(&url, msg, config.clone()) {
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
        let is_push = self.push_flag.clone();
        let cur_block_height = self.get_block_height();
        thread::spawn(move || {
            for url in rx {
                info!("receive url: {:?}", url);
                is_push.write().remove(&url);
                RegisterRecord::save(json!(list).to_string()).unwrap();
            }
            delete_msg(list, queue, cur_block_height);
            debug!("receive end");
        });
    }
}

fn is_post_msg(queue: BlockQueue, height: u64, prefixes: Vec<String>) -> Option<Message> {
    if let Some(msg) = queue.read().get(&height) {
        let mut push_msg = Message::new(height);
        for v in msg {
            let msg_prefix: String = serde_json::from_str(&v["prefix"].to_string()).unwrap();
            for prefix in &prefixes {
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

fn split_msg(msg: Message, slice_num: usize) -> Vec<Message> {
    debug!("slice");
    if msg.data.len() > slice_num {
        let mut v = Vec::new();
        for i in 0..msg.data.len() / slice_num {
            let mut m = Message::new(msg.height);
            for j in 0..slice_num {
                match msg.data.get(i * slice_num + j) {
                    Some(s) => m.add(s.clone()),
                    None => break,
                }
            }
            v.push(m);
        }
        v
    } else {
        vec![msg]
    }
}

fn post_msg(url: &str, msg: Message, config: Config) -> bool {
    debug!("post");
    let slice_msg = split_msg(msg, 10);
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
            std::thread::sleep(config.retry_interval);
        }
        if !flag {
            return false;
        }
    }
    true
}

fn delete_msg(list: RegisterList, queue: BlockQueue, cur_block_height: u64) {
    let mut min_block_height = u64::max_value();
    for register in list.read().unwrap().values() {
        let reg = register.lock().unwrap();
        if !reg.status.down && reg.status.height > 0 && reg.status.height - 1 < min_block_height {
            min_block_height = reg.status.height - 1;
        }
    }

    if min_block_height <= cur_block_height {
        let mut h = *queue.read().keys().next().unwrap();
        info!(
            "height: {:?}, min_block_height: {:?}, len: {:?}",
            h,
            min_block_height,
            queue.read().len()
        );
        while h <= min_block_height {
            match queue.write().remove(&h) {
                Some(_) => info!("del msg: {:?}", h),
                None => warn!("error: no key: {:?}", h),
            };
            h += 1;
        }
    } else {
        warn!("no register!");
        queue.write().clear();
    }
}
