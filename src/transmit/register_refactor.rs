use std::collections::HashMap;
use std::sync::Arc;

use jsonrpc_core::{Result as RpcResult, Error as RpcError};
use jsonrpc_http_server::{
    AccessControlAllowOrigin, DomainsValidation, RestApi, Server, ServerBuilder,
};
use parking_lot::RwLock;
use serde::Deserialize;

use crate::Result;

// HashMap: key - register url; value - register context.
pub type RegisterMap = Arc<RwLock<HashMap<String, RegisterContext>>>;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct RegisterContext {
    /// The prefixes of block storage that required by registrant.
    pub prefix: Vec<String>,
    /// The representation that used to distinguish whether the storage info matches the requirements.
    pub version: String,
    /// The block height of the block that has been pushed.
    pub push_height: u64,
    /// The flag that registrant can handle push message.
    pub is_handling: bool,
}

impl RegisterContext {
    pub fn new(prefix: Vec<String>, version: String) -> Self {
        Self {
            prefix,
            version,
            push_height: 0,
            is_handling: true,
        }
    }

    pub fn add_prefix(&mut self, prefix: String) {
        self.prefix.push(prefix);
    }

    pub fn handle_new_version(&mut self, prefix: &str, version: &str) {
        let prefix = prefix.to_string();
        let version = version.to_string();
        if version > self.version {
            self.prefix.clear();
            self.add_prefix(prefix);
            self.version = version;
        } else {
            if let None = self.prefix.iter().find(|&x| x == &prefix) {
                self.add_prefix(prefix);
            }
        }
        self.is_handling = true;
    }

    pub fn add_height(&mut self) {
        self.push_height += 1;
    }
}

build_rpc_trait! {
    pub trait RegisterApi {
        #[rpc(name = "register")]
        fn register(&self, String, String, String) -> RpcResult<String>;
    }
}

#[derive(Default)]
struct Register(pub RegisterMap);

fn from_json_str<'a, T>(s: &'a str) -> RpcResult<T>
where
    T: Deserialize<'a>,
{
    match serde_json::from_str(s) {
        Ok(value) => Ok(value),
        Err(_) => Err(RpcError::parse_error()),
    }
}

impl RegisterApi for Register {
    fn register(&self, prefix: String, url: String, version: String) -> RpcResult<String> {
        let url: String = from_json_str(&url)?;
        let prefix: String = from_json_str(&prefix)?;
        let version: String = from_json_str(&version)?;
        let register_detail = format!(
            "url: {:?}, prefix: {:?}, version: {:?}",
            &url, &prefix, &version
        );

        self.0
            .write()
            .entry(url)
            .and_modify(|ctxt| ctxt.handle_new_version(&prefix, &version))
            .or_insert_with(|| RegisterContext::new(vec![prefix], version));

        info!("Register Ok: [ {} ]", register_detail);
        Ok("OK".to_string())
    }
}

pub fn build_http_rpc_server(rpc_http: &str) -> Result<(Server, RegisterMap)> {
    let mut io = jsonrpc_core::IoHandler::new();
    let register = Register::default();
    let register_map = register.0.clone();
    io.extend_with(register.to_delegate());

    let server = ServerBuilder::new(io)
        .threads(3)
        .rest_api(RestApi::Unsecure)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Any,
        ]))
        .start_http(&rpc_http.parse().expect("Parse http rpc url"))?;
    Ok((server, register_map))
}

#[cfg(test)]
mod tests {
    extern crate jsonrpc_test;

    use std::thread;
    use std::time::Duration;
    use super::*;

    #[test]
    fn test_single_register_request() {
        let register = Register::default();
        let register_map = register.0.clone();
        let rpc = jsonrpc_test::Rpc::new(register.to_delegate());

        assert_eq!(
            rpc.request(
                "register",
                &[r#""FreeBalance""#, r#""127.0.0.1:12345""#, r#""1.0""#]
            ),
            r#""OK""#
        );

        let map = register_map.read();
        let ctxt = map.get("127.0.0.1:12345").unwrap().clone();
        assert_eq!(
            ctxt,
            RegisterContext {
                prefix: vec!["FreeBalance".to_string()],
                version: "1.0".to_string(),
                push_height: 0,
                is_handling: true,
            }
        );
    }

    #[test]
    fn test_multiple_register_requests() {
        let register = Register::default();
        let register_map = register.0.clone();
        let rpc = jsonrpc_test::Rpc::new(register.to_delegate());

        assert_eq!(
            rpc.request(
                "register",
                &[r#""FreeBalance1""#, r#""127.0.0.1:12345""#, r#""1.0""#]
            ),
            r#""OK""#
        );

        assert_eq!(
            rpc.request(
                "register",
                &[r#""FreeBalance2""#, r#""127.0.0.1:12345""#, r#""1.0""#]
            ),
            r#""OK""#
        );

        let map = register_map.read();
        let ctxt = map.get("127.0.0.1:12345").unwrap().clone();
        assert_eq!(
            ctxt,
            RegisterContext {
                prefix: vec!["FreeBalance1".to_string(), "FreeBalance2".to_string()],
                version: "1.0".to_string(),
                push_height: 0,
                is_handling: true,
            }
        );
    }

    #[test]
    fn test_new_version_register_request() {
        let register = Register::default();
        let register_map = register.0.clone();
        let rpc = jsonrpc_test::Rpc::new(register.to_delegate());

        assert_eq!(
            rpc.request(
                "register",
                &[r#""FreeBalance1""#, r#""127.0.0.1:12345""#, r#""1.0""#]
            ),
            r#""OK""#
        );

        assert_eq!(
            rpc.request(
                "register",
                &[r#""FreeBalance2""#, r#""127.0.0.1:12345""#, r#""2.0""#]
            ),
            r#""OK""#
        );

        assert_eq!(
            rpc.request(
                "register",
                &[r#""FreeBalance3""#, r#""127.0.0.1:12345""#, r#""2.0""#]
            ),
            r#""OK""#
        );

        let map = register_map.read();
        let ctxt = map.get("127.0.0.1:12345").unwrap().clone();
        assert_eq!(
            ctxt,
            RegisterContext {
                prefix: vec!["FreeBalance2".to_string(), "FreeBalance3".to_string()],
                version: "2.0".to_string(),
                push_height: 0,
                is_handling: true,
            }
        );
    }
}
