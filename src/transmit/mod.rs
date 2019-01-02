mod json_manage;
mod push;
mod register;

use std::thread;
use std::time::Duration;

use jsonrpc_http_server::Server;

use transmit::register::{start_rpc, RegisterInfo, RegisterList};
use {BlockQueue, HashMap};

pub fn start(server_url: String, block_queue: BlockQueue) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let register_server = RegisterServer::new(server_url);
        register_server.load();

        let push = push::Client::new(
            register_server.list,
            block_queue,
            push::Config::new(3, Duration::new(3, 0)),
        );
        push.start();

        register_server.server.wait();
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

    pub fn load(&self) {
        if let Ok(Some(string)) = json_manage::read() {
            let res: serde_json::Result<HashMap<String, RegisterInfo>> =
                serde_json::from_str(string.as_str());
            if let Ok(map) = res {
                for (k, v) in map {
                    self.list.write().unwrap().insert(k, v);
                }
            }
        }
    }
}
