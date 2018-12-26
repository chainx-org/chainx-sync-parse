use std::collections::btree_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use error::Result;
use transmit::json_manage;
use transmit::register::{RegisterInfo, RegisterList};
use {Arc, BlockQueue, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    msg_type: String,
    prefix: String,
    key: String,
    value: Vec<String>,
}

impl Message {
    pub fn new(json: serde_json::Value) -> Result<Self> {
        let msg: Message = serde_json::from_value(json)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessage {
    height: u64,
    data: Vec<Message>,
}

impl PushMessage {
    pub fn new(num: u64) -> Self {
        Self {
            height: num,
            data: Default::default(),
        }
    }

    pub fn add(&mut self, data: Message) {
        self.data.push(data);
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
}

impl Client {
    pub fn new(register_list: RegisterList, block_queue: BlockQueue, config: Config) -> Self {
        Self {
            register_list,
            block_queue,
            config,
        }
    }

    pub fn get_block_height(&self) -> u64 {
        match self.block_queue.read().keys().next_back() {
            Some(s) => s.clone(),
            None => 0,
        }
    }

    pub fn start(&self) {
        let msg = vec![
            json!({
            "msg_type": "value",
            "prefix": "block",
            "key": "",
            "value": vec!["11111"],
            }),
            json!({
            "msg_type": "map",
            "prefix": "test2",
            "key": "test3",
            "value": vec!["arg2","arg3"],
            }),
        ];

        let msg_a = msg.clone();
        let queue = self.block_queue.clone();
        thread::spawn(move || {
            let mut num = 0_u64;
            for i in 0..100 {
                println!("add");
                let m = msg_a.get(i % 2).unwrap();
                queue.write().insert(num, m.clone());
                num += 1;
                std::thread::sleep(Duration::new(1, 0));
            }
        });

        let mut flag_map = Arc::new(RwLock::new(HashMap::<String, bool>::default()));
        loop {
            if self.block_queue.read().len() > 0 {
                let cur_block_height = self.get_block_height();
                let (tx, rx) = mpsc::channel();
                let mut is_push = false;
                for register in self.register_list.read().unwrap().iter() {
                    let push_height = register.1.lock().unwrap().status.height;
                    let is_down = register.1.lock().unwrap().status.down;
                    if cur_block_height > push_height && !is_down {
                        if let Vacant(flag) = flag_map.write().entry(register.0.clone()) {
                            println!("{:?}, {:?}, {:?}", cur_block_height, push_height, is_down);
                            flag.insert(true);
                            is_push = true;
                            self.push_msg(
                                cur_block_height,
                                register.0.clone(),
                                register.1.clone(),
                                tx.clone(),
                            );
                        }
                    }
                }

                if is_push {
                    println!("push");
                    let list = self.register_list.clone();
                    let queue = self.block_queue.clone();
                    let block_height = cur_block_height;
                    let mut flag = flag_map.clone();
                    thread::spawn(move || {
                        for rx in rx {
                            let url = rx;
                            println!("{:?}", url);
                            flag.write().remove(&url);
                            json_manage::write(json![list].to_string()).unwrap();
                        }

                        let mut min_block_height = u64::max_value();
                        for register in list.read().unwrap().values() {
                            let reg = register.lock().unwrap();
                            if !reg.status.down {
                                if reg.status.height < min_block_height {
                                    min_block_height = reg.status.height;
                                }
                            }
                        }
                        if min_block_height == u64::max_value() {
                            min_block_height = 0;
                        }

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
                            }
                        }
                    });
                }
            }
        }
    }

    pub fn push_msg(
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
                    if let Some(value) = queue.read().get(&reg.status.height) {
                        let msg = Message::new(value.clone()).unwrap();
                        let mut push_msg = PushMessage::new(reg.status.height);
                        for prefix in &reg.prefix {
                            if *prefix == msg.prefix {
                                println!("{:?},{:?},{:?}", *prefix, msg.prefix, msg.value);
                                push_msg.add(msg.clone());
                            }
                        }
                        if push_msg.data.len() > 0 {
                            println!("post");
                            if post(url.clone(), push_msg, config.clone()) {
                                reg.status.height += 1;
                            } else {
                                reg.status.down = true;
                                break;
                            }
                        } else {
                            reg.status.height += 1;
                        }
                    } else {
                        println!("can not find msg, height: {:?}", reg.status.height);
                        break;
                    }
                }
                tx.send(url.clone()).unwrap();
            };
        });
    }
}

pub fn slice(msg: PushMessage) -> Vec<PushMessage> {
    if msg.data.len() > 10 {
        let mut v = Vec::new();
        for i in 0..msg.data.len() / 10 {
            let mut m = PushMessage::new(msg.height);
            for j in 0..10 {
                match msg.data.get(i * 10 + j) {
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

pub fn post(url: String, msg: PushMessage, config: Config) -> bool {
    let slice_msg = slice(msg);
    for msg in slice_msg {
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
