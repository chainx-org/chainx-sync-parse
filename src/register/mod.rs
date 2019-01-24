mod push;
mod util;

use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use jsonrpc_core::{Params, Result as RpcResult};
use jsonrpc_http_server::{
    AccessControlAllowOrigin, DomainsValidation, RestApi, Server, ServerBuilder,
};
use parking_lot::{Mutex, RwLock};
use serde_json::Value;

use self::push::{Message, PushClient};
use crate::{BlockQueue, Result};

const NUM_THREADS_FOR_REGISTERING: usize = 4;
const NUM_THREADS_FOR_SENDING_REQUEST: usize = 8;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    /// The prefixes of block storage that required by registrant.
    pub prefixes: Vec<String>,
    /// The representation that used to distinguish whether the storage info matches the requirements.
    pub version: String,
    /// The flag that represents whether registrant is normal.
    pub is_normal: bool,
    /// The block height of the block that has been pushed.
    pub push_height: u64,
}

impl Context {
    pub fn new(prefixes: Vec<String>, version: String) -> Self {
        Self {
            prefixes,
            version,
            is_normal: true,
            push_height: 0,
        }
    }

    pub fn add_prefix(&mut self, prefix: String) {
        self.prefixes.push(prefix);
    }

    pub fn handle_existing_url(&mut self, prefix: String, version: String) {
        if version > self.version {
            self.prefixes.clear();
            self.add_prefix(prefix);
            self.version = version;
        } else if self.prefixes.iter().find(|&x| x == &prefix).is_none() {
            self.add_prefix(prefix);
        }
        self.is_normal = true;
    }
}

// HashMap: key - register url; value - register context.
pub type RegisterContext = Arc<Mutex<Context>>;
pub type RegisterMap = Arc<RwLock<HashMap<String, RegisterContext>>>;

pub struct RegisterService {
    /// The block queue (BTreeMap: key - block height, value - json value).
    block_queue: BlockQueue,
    /// The map of register url and register context.
    map: RegisterMap,
}

impl RegisterService {
    pub fn run(&self, url: &str, block_queue: BlockQueue) -> Result<thread::JoinHandle<()>> {
        let (tx, rx) = mpsc::channel();
        self.spawn_remove_block(rx);
        let server = start_http_rpc_server(url, block_queue)?;

        Ok(thread::spawn(move || {
            loop {

            }
            server.wait();
        }))
    }

    pub fn new(block_queue: BlockQueue) -> Self {
        Self {
            block_queue,
            map: Default::default(),
        }
    }

    pub fn spawn_new_push(
        &self,
        url: String,
        ctxt: RegisterContext,
        tx: Sender<(String, u64, bool)>,
    ) -> thread::JoinHandle<()> {
        let queue = self.block_queue.clone();
        let client = PushClient::new(url);
        let push_height = ctxt.lock().push_height;

        thread::spawn(move || 'outer: loop {
            if queue.read().is_empty() {
                thread::sleep(Duration::from_secs(1));
                continue;
            }

            let max_block_height = util::get_max_block_height(&queue);

            for h in push_height..=max_block_height {
                if let Some(values) = queue.read().get(&h) {
                    let msg = Message::build(h, values, &ctxt.lock().prefixes);
                    if !msg.is_empty() && client.post_big_message(msg).is_err() {
                        error!("Terminate the abnormal push thread, url: {:?}", &client.url);
                        // save abnormal register in the disk.
                        tx.send((client.url.clone(), h, false)).unwrap();
                        break 'outer;
                    } else {
                        tx.send((client.url.clone(), h, true)).unwrap();
                    }
                }
            }
        })
    }

    // Spawn a thread for removing pushed block for all register.
    fn spawn_remove_block(&self, rx: Receiver<(String, u64, bool)>) -> thread::JoinHandle<()> {
        info!("Start thread for removing block from queue");
        let queue = self.block_queue.clone();
        thread::spawn(move || {
            let mut stat = HashMap::new();
            loop {
                match rx.try_recv() {
                    Ok(data) => remove_block_from_queue(&queue, &mut stat, data),
                    Err(TryRecvError::Empty) => (),
                    Err(TryRecvError::Disconnected) => {
                        error!("Send half for sending push_height is disconnected");
                        break;
                    }
                }
            }
        })
    }
}

fn remove_block_from_queue(
    queue: &BlockQueue,
    stat: &mut HashMap<String, u64>,
    data: (String, u64, bool),
) {
    let (url, push_height, is_normal) = data;
    if is_normal {
        stat.entry(url)
            .and_modify(|height| *height = push_height)
            .or_insert(push_height);
    } else {
        stat.remove(&url);
    }

    let max_block_height = util::get_max_block_height(queue);
    let min_block_height = util::get_min_block_height(queue);
    let min_push_height = match stat.values().min() {
        Some(height) => *height,
        None => 0,
    };
    info!(
        "min height: [{}], push height: [{}], max height: [{}]",
        min_block_height, min_push_height, max_block_height
    );
    assert!(min_push_height <= max_block_height);
    for h in min_block_height..=min_push_height {
        queue.write().remove(&h);
    }
}

build_rpc_trait! {
    pub trait RegisterApi {
        #[rpc(name = "register")]
        fn register(&self, String, String, String) -> RpcResult<String>;
    }
}

impl RegisterApi for RegisterService {
    fn register(&self, prefix: String, url: String, version: String) -> RpcResult<String> {
        let register_detail = format!(
            "url: {:?}, prefix: {:?}, version: {:?}",
            &url, &prefix, &version
        );

        self.map
            .write()
            .entry(url.clone())
            .and_modify(|ctxt| {
                info!("Existing register: [ {} ]", register_detail);
                ctxt.lock()
                    .handle_existing_url(prefix.clone(), version.clone())
            })
            .or_insert_with(|| {
                info!("New register: [ {} ]", register_detail);
                Arc::new(Mutex::new(Context::new(vec![prefix], version)))
            });

        Ok("OK".to_string())
    }
}

fn start_http_rpc_server(url: &str, block_queue: BlockQueue) -> Result<Server> {
    let mut io = jsonrpc_core::IoHandler::new();
    let register = RegisterService::new(block_queue);
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
        let register = RegisterService::new(BlockQueue::default());
        let register_map = register.map.clone();
        let rpc = jsonrpc_test::Rpc::new(register.to_delegate());

        assert_eq!(
            rpc.request("register", &["FreeBalance", "127.0.0.1:12345", "1.0"]),
            r#""OK""#
        );

        let map = register_map.read();
        let ctxt = map.get("127.0.0.1:12345").unwrap().clone();
        assert_eq!(ctxt.lock().prefixes, vec!["FreeBalance".to_string()]);
        assert_eq!(ctxt.lock().version, "1.0".to_string());
    }

    #[test]
    fn test_multiple_register_requests() {
        let register = RegisterService::new(BlockQueue::default());
        let register_map = register.map.clone();
        let rpc = jsonrpc_test::Rpc::new(register.to_delegate());

        assert_eq!(
            rpc.request("register", &["FreeBalance1", "127.0.0.1:12345", "1.0"]),
            r#""OK""#
        );

        assert_eq!(
            rpc.request("register", &["FreeBalance2", "127.0.0.1:12345", "1.0"]),
            r#""OK""#
        );

        let map = register_map.read();
        let ctxt = map.get("127.0.0.1:12345").unwrap().clone();
        assert_eq!(
            ctxt.lock().prefixes,
            vec!["FreeBalance1".to_string(), "FreeBalance2".to_string()]
        );
        assert_eq!(ctxt.lock().version, "1.0".to_string());
    }

    #[test]
    fn test_new_version_register_request() {
        let register = RegisterService::new(BlockQueue::default());
        let register_map = register.map.clone();
        let rpc = jsonrpc_test::Rpc::new(register.to_delegate());

        assert_eq!(
            rpc.request("register", &["FreeBalance1", "127.0.0.1:12345", "1.0"]),
            r#""OK""#
        );

        assert_eq!(
            rpc.request("register", &["FreeBalance2", "127.0.0.1:12345", "2.0"]),
            r#""OK""#
        );

        assert_eq!(
            rpc.request("register", &["FreeBalance3", "127.0.0.1:12345", "2.0"]),
            r#""OK""#
        );

        let map = register_map.read();
        let ctxt = map.get("127.0.0.1:12345").unwrap().clone();
        assert_eq!(
            ctxt.lock().prefixes,
            vec!["FreeBalance2".to_string(), "FreeBalance3".to_string()]
        );
        assert_eq!(ctxt.lock().version, "2.0".to_string());
    }
}
