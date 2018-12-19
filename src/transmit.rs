use error::Result;
use register_server::jsonrpc_http_server::Server;
use register_server::{Registrant, StartRPC};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

pub struct TransmitClient {
    register_server: RegisterServer,
}

pub struct RegisterServer {
    server: Server,
    registrant: Arc<RwLock<HashMap<String, Arc<Mutex<Registrant>>>>>,
}

impl StartRPC for RegisterServer {}

impl RegisterServer {
    pub fn new(url: &'static str) -> RegisterServer {
        let (server, registrant) = RegisterServer::start_rpc(url);
        RegisterServer {
            server: server,
            registrant: registrant,
        }
    }
}

impl TransmitClient {
    pub fn start(url: &'static str) -> Result<thread::JoinHandle<Result<()>>> {
        let thread = thread::spawn(move || {
            let url = url.clone();
            let reg_server = RegisterServer::new(url);
            reg_server.server.wait();
            Ok(())
        });
        Ok(thread)
    }
}
