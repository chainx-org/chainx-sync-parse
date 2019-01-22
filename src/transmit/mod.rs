mod push;
mod push_refactor;
mod register;
mod register_refactor;
mod rpc;

use std::collections::HashMap;
use std::thread;

use self::push::{Config, PushService};
use self::register::{RegisterInfo, RegisterList, RegisterRecord};
use self::rpc::build_http_rpc_server;
use crate::{BlockQueue, Result};

pub struct RegisterService;

impl RegisterService {
    pub fn run(url: &str, block_queue: BlockQueue) -> Result<thread::JoinHandle<()>> {
        let (server, list) = build_http_rpc_server(url)?;
        Self::load(&list)?;

        let thread = thread::spawn(move || {
            let mut push_service = PushService::new(list, block_queue, Config::default());
            push_service.start();
            server.wait();
        });

        Ok(thread)
    }

    /// Load registrant records from the file `register.json`.
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
