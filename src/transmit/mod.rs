mod push;
mod register;

use std::thread;
use std::time::Duration;

use jsonrpc_http_server::Server;

use crate::{BlockQueue, HashMap, Result};
use self::register::{start_rpc, RegisterInfo, RegisterList, RegisterRecord};

pub fn start(server_url: String, block_queue: BlockQueue) -> thread::JoinHandle<Result<()>> {
    thread::spawn(move || {
        let register_server = RegisterServer::new(server_url);
        register_server.load()?;

        let mut push = push::Client::new(
            register_server.list,
            block_queue,
            push::Config::new(3, Duration::new(3, 0)),
        );
        push.start();
        register_server.server.wait();
        Ok(())
    })
}

pub struct RegisterServer {
    server: Server,
    list: RegisterList,
}

impl RegisterServer {
    pub fn new(url: String) -> Self {
        let (server, list) = start_rpc(url);
        Self { server, list }
    }

    pub fn load(&self) -> Result<()> {
        let string = RegisterRecord::load()?;
        if let Some(string) = string {
            let map: HashMap<String, RegisterInfo> = serde_json::from_str(string.as_str())?;
            for (k, v) in map {
                self.list.write().unwrap().insert(k, v);
            }
        }
        Ok(())
    }
}
