use error::Result;
use register_server::RegistrantList;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use {json_manage, BlockQueue};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    msg_type: String,
    prefix: Option<String>,
    key: String,
    value: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessage {
    height: u64,
    data: Vec<Message>,
}

#[derive(Debug)]
pub struct Client {
    registrant_list: RegistrantList,
    block_queue: BlockQueue,
    config: Config,
}

#[derive(Debug, Clone)]
pub struct Config {
    retry_count: u32,
    retry_interval: Duration,
}

impl Client {
    pub fn new(registrant_list: RegistrantList, block_queue: BlockQueue, config: Config) -> Self {
        Self { registrant_list, block_queue, config }
    }

    //pub fn get(&self, num: u64) {}

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
            if self.block_queue.read().len() > 0 {
                let (tx, rx) = mpsc::channel();
                let registrant_list = self.registrant_list.read().unwrap();
                //let block_queue = self.block_queue.clone();
                //let config = self.config.clone();
                //println!("{:?}", map.len());
                for registrant in registrant_list.iter() {
                    let url = registrant.0.clone();
                    let reg = registrant.1.clone();
                    let tx = tx.clone();
                    let config = self.config.clone();
                    println!("{:?}, {:?}", url, reg.lock().unwrap());
                    thread::spawn(move || {
                        let mut reg = reg.lock().unwrap();
                        if !reg.status.down {
                            let prifixs = reg.info.prifix.clone();
                            for prifix in prifixs {
                                //if *prifix == msg.key {
                                //println!("{:?}", msg.value);
                                reg.status.offset += 1;
                                //}
                            }
                            let json = json!("asd");
                            let asd = Message::new(json);
                            match asd {
                                Ok(asd) => {
                                    let mut msg = PushMessage::new(10);
                                    msg.add(asd);
                                    let json = json!(msg);
                                    let aa: Result<String> =
                                        json_manage::deserialize(url.as_str(), &json);

                                    if post(url, msg, config) {
                                        tx.send(true).unwrap();
                                    } else {
                                        reg.status.down = true;
                                        tx.send(false).unwrap();
                                    }
                                }
                                _ => (),
                            };
                        }
                    });
                }

                let map3 = registrant_list.clone();
                thread::spawn(move || {
                    for rx in rx {
                        if rx {
                            let j = json![map3];
                            json_manage::write(j.to_string());
                        }
                    }
                });
            }
        }
        Ok(())
    }
}

impl Config {
    pub fn new(retry_count: u32, retry_interval: Duration) -> Self {
        Self {
            retry_count,
            retry_interval,
        }
    }
}

impl Message {
    pub fn new(json: serde_json::Value) -> Result<Self> {
        let msg: Message = serde_json::from_value(json)?;
        Ok(msg)
    }
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

pub fn post(url: String, msg: PushMessage, config: Config) -> bool {
    let json = json!(msg);
    for i in 0..config.retry_count {
        match reqwest::Client::new().post(url.as_str()).json(&json).send() {
            Ok(_) => return true,
            Err(_) => {
                std::thread::sleep(config.retry_interval);
            }
        }
    }
    false
}
