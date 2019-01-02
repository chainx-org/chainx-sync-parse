use std::collections::btree_map::Entry::Occupied;
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

type IsPush = Arc<RwLock<HashMap<String, bool>>>;

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    height: u64,
    date: Vec<serde_json::Value>,
}

impl Message {
    pub fn new(num: u64) -> Self {
        Self {
            height: num,
            date: Default::default(),
        }
    }

    pub fn add(&mut self, date: serde_json::Value) {
        self.date.push(date);
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
    is_push: IsPush,
}

impl Client {
    pub fn new(register_list: RegisterList, block_queue: BlockQueue, config: Config) -> Self {
        Self {
            register_list,
            block_queue,
            config,
            is_push: Arc::new(RwLock::new(HashMap::new())),
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
            if self.block_queue.read().len() > 0 {
                continue;
            };
            let cur_block_height = self.get_block_height();
            let (tx, rx) = mpsc::channel();
            let mut have_new_push = false;
            for (url, info) in self.register_list.read().unwrap().iter() {
                let push_height = info.lock().unwrap().status.height;
                let is_down = info.lock().unwrap().status.down;
                if cur_block_height > push_height && !is_down {
                    if let Vacant(flag) = self.is_push.write().entry(url.clone()) {
                        println!("{:?}, {:?}, {:?}", cur_block_height, push_height, is_down);
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
                println!("cur_push_height: {:?}", cur_push_height);
                while reg.status.height < cur_push_height {
                    if let Some(msg) = is_post(queue.clone(), reg.status.height, reg.prefix.clone())
                    {
                        if !post(url.clone(), msg, config.clone()) {
                            reg.set_down(true);
                            break;
                        }
                    }
                    reg.add_height();
                }
                tx.send(url).unwrap();
            };
        });
    }

    fn receive(&self, rx: Receiver<String>) {
        println!("receive");
        let list = self.register_list.clone();
        let queue = self.block_queue.clone();
        let is_push = self.is_push.clone();
        let cur_block_height = self.get_block_height();
        thread::spawn(move || {
            for rx in rx {
                println!("{:?}", rx);
                is_push.write().remove(&rx);
                json_manage::write(json![list].to_string()).unwrap();
            }

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
                let h = queue.read().keys().next().unwrap().clone();
                println!(
                    "height: {:?}, min_block_height: {:?}, len: {:?}",
                    h,
                    min_block_height,
                    queue.read().len()
                );
                for i in h..min_block_height {
                    if let Occupied(msg) = queue.write().entry(i) {
                        msg.remove();
                        println!("del msg, len: {:?}", queue.read().len());
                    }
                }
            }
        });
    }
}

fn is_post(queue: BlockQueue, height: u64, prefixs: Vec<String>) -> Option<Message> {
    match queue.read().get(&height) {
        Some(v) => {
            let mut push_msg = Message::new(height);
            for v in v {
                let msg_prefix: String = serde_json::from_str(&v["prefix"].to_string()).unwrap();
                for prefix in &prefixs {
                    if *prefix == msg_prefix {
                        println!("{:?},{:?}", *prefix, msg_prefix);
                        push_msg.add(v.clone());
                    }
                }
            }

            if push_msg.date.len() > 0 {
                Some(push_msg)
            } else {
                None
            }
        }
        None => panic!("can not find msg, height: {:?}", height),
    }
}

fn slice(msg: Message, slice_num: usize) -> Vec<Message> {
    if msg.date.len() > slice_num {
        let mut v = Vec::new();
        for i in 0..msg.date.len() / slice_num {
            let mut m = Message::new(msg.height);
            for j in 0..slice_num {
                match msg.date.get(i * slice_num + j) {
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
    let slice_msg = slice(msg, 10);
    for msg in slice_msg {
        println!("msg:{:?}", msg);
        let json = json!(msg);
        let mut flag = true;
        for i in 0..config.retry_count {
            let res: Result<String> = json_manage::deserialize(url.as_str(), &json);
            println!("res: {:?}", res);
            flag = match res {
                Ok(ok) => {
                    if ok == "OK" {
                        break;
                    } else {
                        println!("retry: {:?}", i);
                        std::thread::sleep(config.retry_interval);
                        false
                    }
                }
                Err(_) => {
                    println!("retry: {:?}", i);
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
