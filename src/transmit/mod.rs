pub mod file_io;

use error::Result;
use jsonrpc_http_server::Server;
use register_server::{Registrant, StartRPC};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use MsQueue;

pub struct TransmitClient {
    msg_queue: MsQueue<serde_json::Value>,
    server_url: String,
}

pub struct RegisterServer {
    server: Server,
    registrant: Arc<RwLock<HashMap<String, Arc<Mutex<Registrant>>>>>,
}

impl StartRPC for RegisterServer {}

impl RegisterServer {
    pub fn new(url: String) -> Self {
        let (server, registrant) = RegisterServer::start_rpc(url);
        Self {
            server: server,
            registrant: registrant,
        }
    }

    pub fn load(&self) -> Result<()> {
        let string = file_io::read_string()?;
        let map: HashMap<String, Arc<Mutex<Registrant>>> = file_io::deserialize(string)?;

        for iter in map {
            self.registrant.write().unwrap().insert(iter.0, iter.1);
        }
        Ok(())
    }
}

impl TransmitClient {
    pub fn new(url: String, queue: MsQueue<serde_json::Value>) -> Self {
        Self {
            server_url: url,
            msg_queue: queue,
        }
    }

    pub fn start(&self) -> Result<thread::JoinHandle<Result<()>>> {
        let url = self.server_url.clone();
        let thread = thread::spawn(move || {
            let reg_server = RegisterServer::new(url);
            reg_server.load()?;
            reg_server.server.wait();
            Ok(())
        });
        Ok(thread)
    }
}
