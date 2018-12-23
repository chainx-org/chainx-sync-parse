mod push;

use error::Result;
use jsonrpc_http_server::Server;
use register_server::{start_rpc, RegisterData, RegisterList};
use std::thread;
use std::time::Duration;
use {json_manage, BlockQueue, HashMap};

#[derive(Debug)]
pub struct Client {
    block_queue: BlockQueue,
    server_url: String,
}

pub struct RegisterServer {
    server: Server,
    registrant: RegisterList,
}

//impl StartRPC for RegisterServer {}

impl RegisterServer {
    pub fn new(url: String) -> Self {
        let (server, registrant) = start_rpc(url);
        Self { server, registrant }
    }

    pub fn load(&self) -> Result<()> {
        if let Ok(Some(string)) = json_manage::read() {
            let map: HashMap<String, RegisterData> = serde_json::from_str(string.as_str())?;
            for iter in map {
                self.registrant.write().unwrap().insert(iter.0, iter.1);
            }
        }
        Ok(())
    }
}

impl Client {
    pub fn new(server_url: String, block_queue: BlockQueue) -> Self {
        Self {
            server_url,
            block_queue,
        }
    }

    pub fn start(&self) -> Result<thread::JoinHandle<Result<()>>> {
        let url = self.server_url.clone();
        let block_queue = self.block_queue.clone();
        let thread = thread::spawn(move || {
            let register_server = RegisterServer::new(url);
            register_server.load()?;

            let push = push::Client::new(
                register_server.registrant,
                block_queue,
                push::Config::new(10, Duration::new(10, 0)),
            );
            push.start()?;

            register_server.server.wait();
            Ok(())
        });
        Ok(thread)
    }
}
