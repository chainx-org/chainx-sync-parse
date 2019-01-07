use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use error::Result;
use transmit::json_manage;
use transmit::register::{RegisterInfo, RegisterList};
use {Arc, BlockQueue, RwLock};

type PushFlag = Arc<RwLock<HashMap<String, bool>>>;

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

#[derive(Debug)]
pub struct Client {
    register_list: RegisterList,
    block_queue: BlockQueue,
    config: Config,
    push_flag: PushFlag,
}

impl Client {
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
            Some(s) => s.clone(),
            None => 0,
        }
    }

    pub fn start(&mut self) {
        loop {
            if self.block_queue.read().len() <= 0 {
                continue;
            };
            let cur_block_height = self.get_block_height();
            let (tx, rx) = mpsc::channel();
            let mut have_new_push = false;
            for (url, info) in self.register_list.read().unwrap().iter() {
                let push_height = info.lock().unwrap().status.height;
                let is_down = info.lock().unwrap().status.down;
                if cur_block_height >= push_height && !is_down {
                    if let Vacant(flag) = self.push_flag.write().entry(url.clone()) {
                        info!(
                            "cur_height: {:?}, push_height: {:?}, {:?}",
                            cur_block_height, push_height, is_down
                        );
                        flag.insert(true);
                        have_new_push = true;
                        self.push_msg(cur_block_height, url.clone(), info.clone(), tx.clone());
                    }
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
        url: String,
        reg_data: RegisterInfo,
        tx: Sender<String>,
    ) {
        let queue = self.block_queue.clone();
        let config = self.config.clone();
        thread::spawn(move || {
            if let Ok(mut reg) = reg_data.lock() {
                info!("cur_push_height: {:?}", cur_push_height);
                while reg.status.height <= cur_push_height {
                    if let Some(msg) = is_post(queue.clone(), reg.status.height, reg.prefix.clone())
                    {
                        if !post(url.clone(), msg, config.clone()) {
                            info!("post err");
                            reg.set_down(true);
                            break;
                        }
                        info!("post ok");
                    }
                    reg.add_height();
                }
                info!("post end");
                tx.send(url).unwrap();
            };
        });
    }

    fn receive(&self, rx: Receiver<String>) {
        info!("receive");
        let list = self.register_list.clone();
        let queue = self.block_queue.clone();
        let is_push = self.push_flag.clone();
        let cur_block_height = self.get_block_height();
        thread::spawn(move || {
            for rx in rx {
                info!("{:?}", rx);
                is_push.write().remove(&rx);
                json_manage::IO::write(json![list].to_string()).unwrap();
            }
            delete_msg(list, queue, cur_block_height);
            info!("receive end");
        });
    }
}

fn is_post(queue: BlockQueue, height: u64, prefixs: Vec<String>) -> Option<Message> {
    match queue.read().get(&height) {
        Some(msg) => {
            let mut push_msg = Message::new(height);
            for v in msg {
                let msg_prefix: String = serde_json::from_str(&v["prefix"].to_string()).unwrap();
                for prefix in &prefixs {
                    info!("prefix: {:?}, msg_prefix: {:?}", *prefix, msg_prefix);
                    if *prefix == msg_prefix {
                        info!("find prefix");
                        push_msg.add(v.clone());
                    }
                }
            }

            if push_msg.data.len() > 0 {
                Some(push_msg)
            } else {
                None
            }
        }
        None => {
            error!("can not find msg! height: {:?}", height);
            None
        }
    }
}

fn slice(msg: Message, slice_num: usize) -> Vec<Message> {
    info!("slice");
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

fn post(url: String, msg: Message, config: Config) -> bool {
    info!("post");
    let slice_msg = slice(msg, 10);
    for msg in slice_msg {
        info!("msg:{:?}", msg);
        let json = json!(msg);
        let mut flag = true;
        for i in 0..config.retry_count {
            let res: Result<String> = json_manage::request(url.as_str(), &json);
            info!("res: {:?}", res);
            flag = match res {
                Ok(ok) => {
                    if ok == "OK" {
                        break;
                    } else {
                        info!("retry: {:?}", i);
                        std::thread::sleep(config.retry_interval);
                        false
                    }
                }
                Err(_) => {
                    info!("retry: {:?}", i);
                    std::thread::sleep(config.retry_interval);
                    false
                }
            }
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
        if !reg.status.down {
            if reg.status.height > 0 && reg.status.height - 1 < min_block_height {
                min_block_height = reg.status.height - 1;
            }
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
            info!("del: {:?}", h);
            match queue.write().remove(&h) {
                Some(_) => info!("del msg"),
                None => error!("error: no key!"),
            };
            h += 1;
        }
    } else {
        error!("no register!");
        queue.write().clear();
    }
}
