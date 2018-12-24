use error::Result;
use register_server::{RegisterData, RegisterList};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use {json_manage, BlockQueue};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    msg_type: String,
    prefix: String,
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

    pub fn start(&self) -> Result<()> {
        //        let msg = vec![
        //            r#"{
        //                    "msg_type": "value",
        //                    "key": "test1",
        //                    "value": ["arg0","arg1"]
        //                }"#,
        //            r#"{
        //                    "msg_type": "map",
        //                    "prefix": "p",
        //                    "key": "test2",
        //                    "value": ["arg2","arg3"]
        //                }"#,
        //        ];
        //        //let mut block_num = 0_u64;
        //        let msg_a = msg.clone();
        //        let queue = self.block_queue.clone();
        //        thread::spawn(move || {
        //            let mut num = 0_u64;
        //            for i in 0..100 {
        //                //let aaa = num.c
        //                println!("add");
        //                let m = msg_a.get(i % 2).unwrap();
        //                let j = json!(m);
        //                queue.write().insert(num, j);
        //                num += 1;
        //                std::thread::sleep(Duration::new(1, 0));
        //            }
        //        });
        loop {
            let (tx, rx) = mpsc::channel();
            let cur_block_height = self.get_block_height();
            let mut is_push = false;
            for register in self.register_list.read().unwrap().iter() {
                let push_height = register.1.lock().unwrap().status.height;
                let is_down = register.1.lock().unwrap().status.down;
                println!("cur_block_height: {:?}, push_height: {:?}, is_down: {:?}", cur_block_height, push_height, is_down);
                if cur_block_height > push_height && !is_down {
                    is_push = true;
                    self.push_msg(
                        cur_block_height,
                        register.0.clone(),
                        register.1.clone(),
                        tx.clone(),
                    );
                }
            }
            if is_push {
                let list = self.register_list.clone();
                let queue = self.block_queue.clone();
                thread::spawn(move || {
                    let mut push_num = 0;
                    for rx in rx {
                        if rx {
                            json_manage::write(json![list].to_string());
                            push_num += 1;
                        }
                    }
                    let mut reg_num = 0;
                    for register in list.read().unwrap().values() {
                        if !register.lock().unwrap().status.down {
                            reg_num += 1;
                        }
                    }
                    if push_num == reg_num {
                        queue.write().remove(&cur_block_height);
                    }
                });
            }
        }
        Ok(())
    }

    pub fn push_msg(
        &self,
        cur_push_height: u64,
        url: String,
        reg_data: RegisterData,
        tx: Sender<bool>,
    ) {
        let queue = self.block_queue.clone();
        let config = self.config.clone();
        thread::spawn(move || {
            if let Ok(mut reg) = reg_data.lock() {
                while reg.status.height < cur_push_height {
                    if let Some(value) = queue.read().get(&reg.status.height) {
                        let msg = Message::new(value.clone()).unwrap();
                        let mut push_msg = PushMessage::new(reg.status.height);
                        for prefix in &reg.prefix {
                            if *prefix == msg.prefix {
                                println!("{:?}", msg.value);
                                push_msg.add(msg.clone());
                            }
                        }
                        if post(url.clone(), push_msg, config.clone()) {
                            tx.send(true).unwrap();
                            reg.status.height += 1;
                        } else {
                            reg.status.down = true;
                            tx.send(false).unwrap();
                        }
                    }
                }
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
