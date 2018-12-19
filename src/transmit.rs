use std::thread;
use error::Result;
use register_server::{StartRPC, Registrant};
use register_server::jsonrpc_http_server::Server;
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;

pub struct TransmitClient{
    register_server: RegisterServer,
}

pub struct RegisterServer {
    server: Server,
    registrant: Arc<RwLock<HashMap<String, Arc<Mutex<Registrant>>>>>,
}

impl StartRPC for RegisterServer{}

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