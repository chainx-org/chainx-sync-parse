use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use jsonrpc_core::Result as RpcResult;
use jsonrpc_http_server::{
    AccessControlAllowOrigin, DomainsValidation, RestApi, Server, ServerBuilder,
};

use crate::{Arc, HashMap, Result, StdMutex, StdRwLock};

const REGISTER_RECORD_PATH: &str = "./target/reg.json";

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
            status: Status::default(),
            version,
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

    pub fn set_down(&mut self, down: bool) -> bool {
        if self.status.down != down {
            self.status.down = down;
            true
        } else {
            false
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
        fn register(&self, String, String, String) -> RpcResult<String>;
    }
}

#[derive(Default)]
pub struct RpcImpl {
    register_list: RegisterList,
}

impl Rpc for RpcImpl {
    fn register(&self, prefix: String, url: String, version: String) -> RpcResult<String> {
        let prefix: String = serde_json::from_str(&prefix).unwrap();
        info!("prefix:{:?}, url:{:?}, version{:?}", prefix, url, version);
        let mut new_info = true;
        self.register_list
            .write()
            .unwrap()
            .entry(url)
            .and_modify(|info| {
                let mut info = info.lock().unwrap();
                info!(
                    "version:{:?}, reg_version{:?}",
                    version.parse::<f64>().unwrap(),
                    info.version.parse::<f64>().unwrap()
                );
                if version.parse::<f64>().unwrap() > info.version.parse::<f64>().unwrap() {
                    info.new_version(version.clone());
                    info.add_prefix(prefix.clone());
                } else {
                    match info.prefix.iter().find(|&x| x == &prefix) {
                        Some(_) => new_info = info.set_down(false),
                        None => info.add_prefix(prefix.clone()),
                    }
                }
            })
            .or_insert(Arc::new(StdMutex::new(Info::new(vec![prefix], version))));

        if new_info {
            info!("register ok");
            RegisterRecord::save(json!(self.register_list).to_string()).unwrap();
            Ok("OK".to_string())
        } else {
            info!("register null");
            Ok("NULL".to_string())
        }
    }
}

pub fn build_http_rpc_server(rpc_http: &str) -> (Server, RegisterList) {
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

pub struct RegisterRecord;

impl RegisterRecord {
    pub fn save(json: String) -> Result<()> {
        let p = Path::new(REGISTER_RECORD_PATH);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&p)?;

        file.write(json.as_bytes())?;
        Ok(())
    }

    pub fn load() -> Result<String> {
        let path = Path::new(REGISTER_RECORD_PATH);
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(buf)
    }
}
