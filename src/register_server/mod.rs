use jsonrpc_core::Result;
use jsonrpc_http_server::{
    AccessControlAllowOrigin, DomainsValidation, RestApi, Server, ServerBuilder,
};
use {Arc, HashMap, StdMutex, StdRwLock};

pub type RegistrantData = Arc<StdMutex<Registrant>>;
pub type RegistrantList = Arc<StdRwLock<HashMap<String, RegistrantData>>>;

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
    registrant_list: RegistrantList,
}

impl Rpc for RpcImpl {
    fn register(&self, prifix: String, url: String, version: String) -> Result<String> {
        let reg: RegistrantData = Arc::new(StdMutex::new(Registrant::new(vec![prifix], version)));
        self.registrant_list.write().unwrap().insert(url, reg);
        println!("{:?}", self.registrant_list.read().unwrap());
        Ok("OK".to_string())
    }
}

impl Info {
    pub fn new(prifix: Vec<String>, version: String) -> Self {
        Self { prifix, version }
    }
}

impl Status {
    pub fn new() -> Self {
        Self {
            offset: 0,
            down: false,
        }
    }
}

impl Registrant {
    pub fn new(prifix: Vec<String>, version: String) -> Self {
        Self {
            info: Info::new(prifix, version),
            status: Status::new(),
        }
    }
}

pub trait StartRPC {
    fn start_rpc(rpc_http: String) -> (Server, RegistrantList) {
        let mut io = jsonrpc_core::IoHandler::new();
        let rpc = RpcImpl::default();
        let registrant_list = rpc.registrant_list.clone();
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
}
