use std::sync::{Arc, Mutex};

use jsonrpc_core::Result as RpcResult;
use jsonrpc_http_server::{
    AccessControlAllowOrigin, DomainsValidation, RestApi, Server, ServerBuilder,
};

use super::register::{Info, RegisterList, RegisterRecord};
use crate::Result;

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
        info!(
            "Register: [ url: {:?}, version: {:?}, prefix: {:?}  ]",
            url, version, prefix,
        );
        let prefix: String =
            serde_json::from_str(&prefix).expect("Register prefix deserialize error");
        info!("Deserialize prefix: {:?}", prefix);
        self.register_list
            .write()
            .unwrap()
            .entry(url)
            .and_modify(|info| {
                let mut info = info.lock().unwrap();
                if version.parse::<f64>().unwrap() > info.version.parse::<f64>().unwrap() {
                    info.new_version(version.clone());
                    info.add_prefix(prefix.clone());
                } else {
                    match info.prefix.iter().find(|&x| x == &prefix) {
                        Some(_) => info.switch_on(),
                        None => info.add_prefix(prefix.clone()),
                    }
                }
            })
            .or_insert_with(|| Arc::new(Mutex::new(Info::new(vec![prefix], version))));

        if let Err(err) = RegisterRecord::save(&json!(self.register_list).to_string()) {
            error!("Save register record error: {}", err);
        }
        info!("Register: ok");
        Ok("OK".to_string())
    }
}

pub fn build_http_rpc_server(rpc_http: &str) -> Result<(Server, RegisterList)> {
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
        .start_http(&rpc_http.parse().unwrap())?;
    Ok((server, registrant_list))
}
