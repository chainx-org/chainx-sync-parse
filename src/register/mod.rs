mod push;
mod util;

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use jsonrpc_core::Result as RpcResult;
use jsonrpc_http_server::{
    AccessControlAllowOrigin, DomainsValidation, RestApi, Server, ServerBuilder,
};
use parking_lot::RwLock;

use self::push::{Config, Message, PushClient};
use crate::{BlockQueue, Result};

const NUM_THREADS_FOR_REGISTERING: usize = 4;
const NUM_THREADS_FOR_SENDING_REQUEST: usize = 8;

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

    pub fn handle_existing_url(&mut self, prefix: &str, version: &str) {
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

    //    pub fn add_height(&mut self) {
    //        self.push_height += 1;
    //    }
}

pub struct Register {
    /// The block queue (BTreeMap: key - block height, value - json value).
    block_queue: BlockQueue,
    /// The map of register url and register context.
    map: RegisterMap,
    /// The thread pool for posting JSON-RPC request.
    pool: rayon::ThreadPool,
}

impl Register {
    pub fn run_service(url: &str, block_queue: BlockQueue) -> Result<thread::JoinHandle<()>> {
        let server = start_http_rpc_server(url, block_queue)?;
        Ok(thread::spawn(move || server.wait()))
    }

    pub fn new(block_queue: BlockQueue) -> Self {
        Self {
            block_queue,
            map: Arc::new(RwLock::new(HashMap::new())),
            pool: rayon::ThreadPoolBuilder::new()
                .num_threads(NUM_THREADS_FOR_SENDING_REQUEST)
                .build()
                .unwrap(),
        }
    }

    /// Get the max key of BTreeMap, which is current block height.
    fn get_block_height(&self) -> u64 {
        match self.block_queue.read().keys().next_back() {
            Some(s) => *s,
            None => 0,
        }
    }

    fn remove_from_queue(&self) {
        let mut min_push_height = u64::max_value();
        for ctxt in self.map.read().values() {
            if ctxt.is_handling && ctxt.push_height < min_push_height {
                min_push_height = ctxt.push_height - 1;
            }
        }

        if min_push_height <= self.get_block_height() {}
    }
}

build_rpc_trait! {
    pub trait RegisterApi {
        #[rpc(name = "register")]
        fn register(&self, String, String, String) -> RpcResult<String>;
    }
}

impl RegisterApi for Register {
    fn register(&self, prefix: String, url: String, version: String) -> RpcResult<String> {
        let url: String = util::from_json_str(&url)?;
        let prefix: String = util::from_json_str(&prefix)?;
        let version: String = util::from_json_str(&version)?;
        let register_detail = format!(
            "url: {:?}, prefix: {:?}, version: {:?}",
            &url, &prefix, &version
        );

        self.map
            .write()
            .entry(url.clone())
            .and_modify(|ctxt| ctxt.handle_existing_url(&prefix, &version))
            .or_insert_with(|| RegisterContext::new(vec![prefix], version));

        //                let ctxt = value.clone();
        //                let client = self.client.clone();
        //                let block_queue = self.block_queue.clone();
        //                let map = self.map.clone();
        //                self.pool.spawn(move || loop {
        //                    if block_queue.read().is_empty() {
        //                        continue;
        //                    }
        //                    let cur_block_height = util::get_max_height(&block_queue);
        //                    if cur_block_height >= ctxt.push_height && ctxt.is_handling {
        //
        //                    }
        //                });

        info!("Register Ok: [ {} ]", register_detail);
        Ok("OK".to_string())
    }
}

pub fn start_http_rpc_server(url: &str, block_queue: BlockQueue) -> Result<Server> {
    let mut io = jsonrpc_core::IoHandler::new();
    let register = Register::new(block_queue);
    io.extend_with(register.to_delegate());

    let server = ServerBuilder::new(io)
        .threads(NUM_THREADS_FOR_REGISTERING)
        .rest_api(RestApi::Unsecure)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Any,
        ]))
        .start_http(&url.parse()?)?;
    Ok(server)
}

#[cfg(test)]
mod tests {
    extern crate jsonrpc_test;

    use super::*;

    #[test]
    fn test_single_register_request() {
        let register = Register::new(BlockQueue::default());
        let register_map = register.map.clone();
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
        let register = Register::new(BlockQueue::default());
        let register_map = register.map.clone();
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
        let register = Register::new(BlockQueue::default());
        let register_map = register.map.clone();
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
