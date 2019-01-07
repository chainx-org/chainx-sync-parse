use jsonrpc_core::Result;
use jsonrpc_http_server::{
    AccessControlAllowOrigin, DomainsValidation, RestApi, Server, ServerBuilder,
};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use transmit::json_manage;
use {Arc, HashMap, StdMutex, StdRwLock};

pub type RegisterInfo = Arc<StdMutex<Info>>;
pub type RegisterList = Arc<StdRwLock<HashMap<String, RegisterInfo>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub prefix: Vec<String>,
    pub status: Status,
    version: String,
}

impl Info {
    pub fn new(prefix: Vec<String>, version: String) -> Self {
        Self {
            prefix,
            version,
            status: Status::default(),
        }
    }

    pub fn new_version(&mut self, version: String) {
        self.prefix.clear();
        self.status = Status::default();
        self.version = version;
        self.set_down(false);
    }

    pub fn add_prefix(&mut self, prefix: String) {
        self.prefix.push(prefix);
        self.set_down(false);
    }

    pub fn add_height(&mut self) {
        self.status.height += 1;
    }

    pub fn set_down(&mut self, down: bool) {
        if self.status.down != down {
            self.status.down = down;
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub height: u64,
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
    register_list: RegisterList,
}

impl Rpc for RpcImpl {
    fn register(&self, prefix: String, url: String, version: String) -> Result<String> {
        let prefix: String = serde_json::from_str(&prefix).unwrap();
        info!("prefix:{:?}, url:{:?}, version{:?}", prefix, url, version);
        if let Ok(mut list) = self.register_list.write() {
            match list.entry(url) {
                Vacant(reg) => {
                    reg.insert(Arc::new(StdMutex::new(Info::new(vec![prefix], version))));
                }
                Occupied(reg) => {
                    if let Ok(mut reg) = reg.into_mut().lock() {
                        info!(
                            "version:{:?}, reg_version{:?}",
                            version.parse::<f64>().unwrap(),
                            reg.version.parse::<f64>().unwrap()
                        );
                        if version.parse::<f64>().unwrap() > reg.version.parse::<f64>().unwrap() {
                            reg.new_version(version);
                            reg.add_prefix(prefix);
                        } else {
                            if let None = reg.prefix.iter().find(|&x| x == &prefix) {
                                reg.add_prefix(prefix);
                            } else {
                                if reg.status.down {
                                    reg.set_down(false);
                                } else {
                                    info!("register null");
                                    return Ok("null".to_string());
                                }
                            }
                        }
                    }
                }
            };
        }
        info!("register ok");
        json_manage::write(json!(self.register_list).to_string()).unwrap();
        Ok("OK".to_string())
    }
}

pub fn start_rpc(rpc_http: String) -> (Server, RegisterList) {
    let mut io = jsonrpc_core::IoHandler::new();
    let rpc = RpcImpl::default();
    let registrant_list = rpc.register_list.clone();
    io.extend_with(rpc.to_delegate());

    let server = ServerBuilder::new(io)
        .threads(3)
        .rest_api(RestApi::Unsecure)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Any,
        ]))
        .start_http(&rpc_http.parse().unwrap())
        .expect("Unable to start RPC server");
    (server, registrant_list)
}
