mod push;
mod register;

use std::thread;
use std::time::Duration;

use self::push::{Config, PushClient};

use self::register::{build_http_rpc_server, RegisterInfo, RegisterList, RegisterRecord};
use crate::{BlockQueue, HashMap, Result};

pub struct RegisterService;

impl RegisterService {
    pub fn run(url: &str, block_queue: BlockQueue) -> Result<thread::JoinHandle<()>> {
        let (server, list) = build_http_rpc_server(url);
        Self::load(&list)?;

        let thread = thread::spawn(move || {
            let mut push_client =
                PushClient::new(list, block_queue, Config::new(3, Duration::new(3, 0)));
            push_client.start();
            server.wait();
        });

        Ok(thread)
    }

    fn load(list: &RegisterList) -> Result<()> {
        if let Some(record) = RegisterRecord::load()? {
            let map: HashMap<String, RegisterInfo> = serde_json::from_str(record.as_str())?;
            for (k, v) in map {
                list.write().unwrap().insert(k, v);
            }
        }
        Ok(())
    }
}
