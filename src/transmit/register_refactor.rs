use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use jsonrpc_core::{Error as RpcError, Result as RpcResult};
use jsonrpc_http_server::{
    AccessControlAllowOrigin, DomainsValidation, RestApi, Server, ServerBuilder,
};
use parking_lot::RwLock;
use serde::de::{Deserialize, DeserializeOwned};

use crate::{BlockQueue, Result};

const MSG_CHUNK_SIZE_LIMIT: usize = 10;
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

#[derive(Clone, Copy, Debug)]
pub struct Config {
    retry_count: u32,
    retry_interval: Duration,
}

impl Config {
    pub fn new(retry_count: u32, retry_interval: Duration) -> Self {
        Self {
            retry_count,
            retry_interval,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(3, Duration::new(3, 0))
    }
}

#[derive(PartialEq, Clone, Debug, Serialize)]
pub struct Message {
    height: u64,
    data: Vec<serde_json::Value>,
}

impl Message {
    pub fn new(height: u64) -> Self {
        Self {
            height,
            data: vec![],
        }
    }

    pub fn add(&mut self, value: serde_json::Value) {
        self.data.push(value);
    }

    /// Split the message into multiple messages according to `chunk_size`.
    pub fn split(self, chunk_size: usize) -> Vec<Self> {
        debug!("The message was split into multiple messages");
        let chunks = self
            .data
            .chunks(chunk_size)
            .map(|value| value.to_vec())
            .collect::<Vec<Vec<serde_json::Value>>>();
        chunks
            .into_iter()
            .map(|chunk| Message {
                height: self.height,
                data: chunk,
            })
            .collect()
    }
}

struct Register {
    /// The map of register url and register context.
    map: RegisterMap,
    /// The block queue (BTreeMap: key - block height, value - json value).
    block_queue: BlockQueue,
    /// The thread pool for handling request.
    pool: rayon::ThreadPool,
    /// The client for pushing JSON-RPC request.
    client: PushClient,
}

impl Register {
    fn new(block_queue: BlockQueue) -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
            block_queue,
            client: PushClient::new(),
            pool: rayon::ThreadPoolBuilder::new()
                .num_threads(NUM_THREADS_FOR_SENDING_REQUEST)
                .build()
                .unwrap(),
        }
    }

    fn handle_register(&self, url: String, prefix: String, version: String) {
        self.map
            .write()
            .entry(url)
            .and_modify(|ctxt| ctxt.handle_new_version(&prefix, &version))
            .or_insert_with(|| RegisterContext::new(vec![prefix], version));
        let client = self.client.clone();
        self.pool.install(|| loop {

        });
    }

    /// Get the max key of BTreeMap, which is current block height.
    fn get_block_height(&self) -> u64 {
        match self.block_queue.read().keys().next_back() {
            Some(s) => *s,
            None => 0,
        }
    }
}

#[derive(Debug, Deserialize)]
struct JsonResponse<T> {
    result: T,
}

#[derive(Clone)]
struct PushClient {
    /// The http rpc client for sending JSON-RPC request.
    client: reqwest::Client,
    /// The config of sending JSON-RPC request.
    config: Config,
}

impl PushClient {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            config: Config::default(),
        }
    }

    fn request<T>(&self, url: &str, body: &serde_json::Value) -> Result<T>
    where
        T: Debug + DeserializeOwned,
    {
        let resp: serde_json::Value = self
            .client
            .post(url)
            .json(body)
            .send()?
            .json::<serde_json::Value>()?;
        let resp: JsonResponse<T> = serde_json::from_value(resp)?;
        Ok(resp.result)
    }

    fn request_with_config(&self, url: &str, msg: Message) -> Result<()> {
        let body = json!(msg);
        debug!("Send message request: {:?}", body);
        for i in 1..=self.config.retry_count {
            let ok = self.request::<String>(url, &body)?;
            info!("Receive message response: {:?}", ok);
            if ok == "OK" {
                return Ok(());
            }
            warn!(
                "Send message request retry ({} / {})",
                i, self.config.retry_count
            );
            thread::sleep(self.config.retry_interval);
        }
        warn!(
            "Reach the limitation of retries, failed to send message: {:?}",
            msg
        );
        Err("Reach the limitation of retries".into())
    }
}

fn from_json_str<'a, T>(s: &'a str) -> RpcResult<T>
where
    T: Deserialize<'a>,
{
    match serde_json::from_str(s) {
        Ok(value) => Ok(value),
        Err(_) => Err(RpcError::parse_error()),
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
        let url: String = from_json_str(&url)?;
        let prefix: String = from_json_str(&prefix)?;
        let version: String = from_json_str(&version)?;
        let register_detail = format!(
            "url: {:?}, prefix: {:?}, version: {:?}",
            &url, &prefix, &version
        );

        self.map
            .write()
            .entry(url)
            .and_modify(|ctxt| ctxt.handle_new_version(&prefix, &version))
            .or_insert_with(|| RegisterContext::new(vec![prefix], version));

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
    use std::thread;
    use std::time::Duration;

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

    #[test]
    fn test_message_split() {
        macro_rules! value {
            ($v:expr) => {
                serde_json::from_str::<serde_json::Value>($v).unwrap()
            };
        }

        let message = Message {
            height: 123,
            data: vec![
                value!("1"),
                value!("2"),
                value!("3"),
                value!("4"),
                value!("5"),
            ],
        };

        assert_eq!(
            vec![
                Message {
                    height: 123,
                    data: vec![value!("1"), value!("2")]
                },
                Message {
                    height: 123,
                    data: vec![value!("3"), value!("4")]
                },
                Message {
                    height: 123,
                    data: vec![value!("5")]
                },
            ],
            message.clone().split(2)
        );

        assert_eq!(vec![message.clone()], message.split(5));
    }
}
