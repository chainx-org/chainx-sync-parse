use std::sync::{Arc, Mutex};

use jsonrpc_core::Result as RpcResult;
use jsonrpc_http_server::{
    AccessControlAllowOrigin, DomainsValidation, RestApi, Server, ServerBuilder,
};

use super::register::{Info, RegisterList, RegisterRecord};
use crate::Result;

build_rpc_trait! {
    pub trait RegisterApi {
        #[rpc(name = "register")]
        fn register(&self, String, String, String) -> RpcResult<String>;
    }
}

#[derive(Default)]
struct Registers(pub RegisterList);

impl RegisterApi for Registers {
    fn register(&self, prefix: String, url: String, version: String) -> RpcResult<String> {
        info!(
            "Register: [ url: {:?}, version: {:?}, prefix: {:?}  ]",
            url, version, prefix,
        );
        self.0
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

        if let Err(err) = RegisterRecord::save(&json!(self.0).to_string()) {
            error!("Save register record error: {}", err);
        }
        info!("Register: ok");
        Ok("OK".to_string())
    }
}

pub fn build_http_rpc_server(rpc_http: &str) -> Result<(Server, RegisterList)> {
    let mut io = jsonrpc_core::IoHandler::new();
    let register = Registers::default();
    let register_list = register.0.clone();
    io.extend_with(register.to_delegate());

    let server = ServerBuilder::new(io)
        .threads(3)
        .rest_api(RestApi::Unsecure)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Any,
        ]))
        .start_http(&rpc_http.parse().unwrap())?;
    Ok((server, register_list))
}

#[cfg(test)]
mod tests {
    extern crate jsonrpc_test;

    use super::*;
    use crate::transmit::register::{Info, Status};

    #[test]
    fn test_register() {
        let registers = Registers::default();
        let list = registers.0.clone();
        let rpc = jsonrpc_test::Rpc::new(registers.to_delegate());
        assert_eq!(
            rpc.request("register", &["FreeBalance", "127.0.0.1:12345", "1.0"]),
            r#""OK""#
        );

        let list = list.read().unwrap();
        let info = list.get("127.0.0.1:12345").unwrap().lock().unwrap().clone();
        assert_eq!(
            info,
            Info {
                prefix: vec!["FreeBalance".to_string()],
                status: Status {
                    height: 0,
                    down: false
                },
                version: "1.0".to_string(),
            }
        )
    }
}
