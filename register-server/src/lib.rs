extern crate jsonrpc_core;
pub extern crate jsonrpc_http_server;
#[macro_use]
extern crate jsonrpc_macros;
#[macro_use]
extern crate serde_derive;

use jsonrpc_core::Result;
use jsonrpc_http_server::{
    AccessControlAllowOrigin, DomainsValidation, RestApi, Server, ServerBuilder,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Registrant {
    pub info: Info,
    pub status: Status,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub prifix: Vec<String>,
    pub version: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub offset: u64,
    pub down: bool,
}

build_rpc_trait! {
    pub trait Rpc {
        #[rpc(name = "register")]
        fn register(&self, String, String, String) -> Result<String>;
    }
}

#[derive(Default)]
pub struct RpcImpl {
    registrant_map: Arc<RwLock<HashMap<String, Arc<Mutex<Registrant>>>>>,
}

impl Rpc for RpcImpl {
    fn register(&self, prifix: String, url: String, version: String) -> Result<String> {
        let reg = Arc::new(Mutex::new(Registrant::new(vec![prifix], version)));
        self.registrant_map.write().unwrap().insert(url, reg);
        Ok("OK".to_string())
    }
}

impl Info {
    pub fn new(prifix: Vec<String>, version: String) -> Info {
        Info {
            prifix: prifix,
            version: version,
        }
    }
}

impl Status {
    pub fn new() -> Status {
        Status {
            offset: 0,
            down: false,
        }
    }
}

impl Registrant {
    pub fn new(prifix: Vec<String>, version: String) -> Registrant {
        Registrant {
            info: Info::new(prifix, version),
            status: Status::new(),
        }
    }
}

pub trait StartRPC {
    fn start_rpc(rpc_http: &'static str) -> (Server, Arc<RwLock<HashMap<String, Arc<Mutex<Registrant>>>>>) {
        let mut io = jsonrpc_core::IoHandler::new();
        let rpc = RpcImpl::default();
        let registrant_map = rpc.registrant_map.clone();
        io.extend_with(rpc.to_delegate());

        let server = ServerBuilder::new(io)
            .threads(3)
            .rest_api(RestApi::Unsecure)
            .cors(DomainsValidation::AllowOnly(vec![
                AccessControlAllowOrigin::Any,
            ]))
            .start_http(&rpc_http.parse().unwrap())
            .expect("Unable to start RPC server");
        (server, registrant_map)
    }
}
