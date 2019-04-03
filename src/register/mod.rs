mod push;
mod util;

use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use jsonrpc_derive::rpc;
use parking_lot::{Mutex, RwLock};

use self::push::{Message, PushClient};
use crate::{BlockQueue, Result};

const NUM_THREADS_FOR_REGISTERING: usize = 4;

#[derive(PartialEq, Clone, Debug)]
struct Context {
    /// The prefixes of block storage that required by registrant.
    pub prefixes: Vec<String>,
    /// The representation that used to distinguish whether the storage info matches the requirements.
    pub version: String,
    /// The block height of the block that has been pushed.
    pub push_height: u64,
    /// The flag the represents whether registrant deregister.
    pub deregister: bool,
}

impl Context {
    pub fn new(prefixes: Vec<String>, version: String) -> Self {
        Self {
            prefixes,
            version,
            push_height: 0,
            deregister: false,
        }
    }

    pub fn handle_existing_url(&mut self, prefix: String, version: String) {
        if version > self.version {
            self.prefixes.clear();
            self.add_prefix(prefix);
            self.version = version;
            info!(
                "New version: [{}], new prefixes: [{:?}]",
                &self.version, &self.prefixes
            );
        } else if self.prefixes.iter().find(|&x| x == &prefix).is_none() {
            self.add_prefix(prefix);
            info!("New prefixes: [{:?}]", &self.prefixes);
        }
    }

    fn add_prefix(&mut self, prefix: String) {
        self.prefixes.push(prefix);
    }
}

// HashMap: key - register url; value - register context.
type RegisterMap = Arc<RwLock<HashMap<String, RegisterContext>>>;
type RegisterContext = Arc<Mutex<Context>>;

type PushSender = Sender<NotifyData>;
type PushReceiver = Receiver<NotifyData>;

enum NotifyData {
    Normal((String, u64)),
    Abnormal(String),
}

/// Register API
#[rpc]
pub trait RegisterApi {
    /// Register
    #[rpc(name = "register")]
    fn register(&self, prefix: String, url: String, version: String) -> Result<String>;

    /// Deregister
    #[rpc(name = "deregister")]
    fn deregister(&self, url: String) -> Result<String>;
}

pub struct RegisterService {
    /// The block queue (BTreeMap: key - block height, value - json value).
    block_queue: BlockQueue,
    /// The map of register url and register context.
    map: RegisterMap,
    /// PushData sender
    tx: Mutex<PushSender>,
}

impl RegisterService {
    pub fn new(block_queue: BlockQueue) -> Self {
        let (tx, rx) = mpsc::channel();
        let service = RegisterService {
            block_queue,
            map: Default::default(),
            tx: Mutex::new(tx),
        };
        service.spawn_remove_block(rx);
        service
    }

    pub fn run(self, url: &str) -> Result<jsonrpc_http_server::Server> {
        let io = rpc_handler(self);
        start_http_rpc_server(url, io)
    }

    fn spawn_new_push(&self, url: String, ctxt: RegisterContext, tx: PushSender) {
        let queue = self.block_queue.clone();
        let client = PushClient::new(url.clone());
        info!("Register: start push thread of url: [{}]", &client.url);

        thread::spawn(move || 'outer: loop {
            if ctxt.lock().deregister {
                tx.send(NotifyData::Abnormal(client.url.clone()))
                    .expect("Unable to send context");
                warn!("Deregister: [{}] 's push thread will terminate", &url);
                break 'outer;
            }
            // Ensure that there is at least one block in the queue.
            if queue.read().len() <= 1 {
                thread::sleep(Duration::from_secs(1));
                continue;
            }
            let push_height = ctxt.lock().push_height;
            let max_block_height = util::get_max_block_height(&queue);
            for h in push_height..max_block_height {
                let msg = match queue.read().get(&h) {
                    Some(values) => Message::build(h, values, &ctxt.lock().prefixes),
                    None => Message::empty(h),
                };
                if !msg.is_empty() && client.post_big_message(msg).is_err() {
                    tx.send(NotifyData::Abnormal(client.url.clone()))
                        .expect("Unable to send context");
                    warn!("Post abnormal: [{}] 's push thread will terminate", &url);
                    break 'outer;
                } else {
                    ctxt.lock().push_height = h + 1; // next push height
                    tx.send(NotifyData::Normal((client.url.clone(), h)))
                        .expect("Unable to send context");
                }
            }
        });
    }

    // Spawn a thread for removing pushed block for all register.
    fn spawn_remove_block(&self, rx: PushReceiver) {
        let queue = self.block_queue.clone();
        let map = self.map.clone();
        thread::spawn(move || {
            info!("Register service starts thread for removing block from queue");
            let mut stat = HashMap::new();
            loop {
                match rx.try_recv() {
                    Ok(data) => remove_block_from_queue(&queue, &mut stat, &map, data),
                    Err(TryRecvError::Empty) => thread::sleep(Duration::from_millis(50)),
                    Err(TryRecvError::Disconnected) => {
                        error!("Register: remove block thread terminated");
                        break;
                    }
                }
            }
        });
    }
}

impl RegisterApi for RegisterService {
    fn register(&self, prefix: String, url: String, version: String) -> Result<String> {
        let register_detail = format!(
            "url: {:?}, prefix: {:?}, version: {:?}",
            &url, &prefix, &version
        );
        self.map
            .write()
            .entry(url.clone())
            .and_modify(|ctxt| {
                info!("Register existing [{}]", register_detail);
                ctxt.lock()
                    .handle_existing_url(prefix.clone(), version.clone())
            })
            .or_insert_with(|| {
                info!("Register new [{}]", register_detail);
                let tx = self.tx.lock().clone();
                let ctxt = Arc::new(Mutex::new(Context::new(vec![prefix], version)));
                self.spawn_new_push(url, ctxt.clone(), tx);
                ctxt
            });
        Ok("OK".to_string())
    }

    fn deregister(&self, url: String) -> Result<String> {
        use std::collections::hash_map::Entry;
        match self.map.write().entry(url.clone()) {
            Entry::Occupied(mut entry) => {
                info!("Deregister [{}]", url);
                let ctxt = entry.get_mut();
                ctxt.lock().deregister = true;
                Ok("OK".to_string())
            }
            Entry::Vacant(_) => Err("Nonexistent register url".into()),
        }
    }
}

fn remove_block_from_queue(
    queue: &BlockQueue,
    stat: &mut HashMap<String, u64>,
    map: &RegisterMap,
    data: NotifyData,
) {
    match data {
        NotifyData::Normal((url, push_height)) => {
            stat.entry(url)
                .and_modify(|height| *height = push_height)
                .or_insert(push_height);
        }
        NotifyData::Abnormal(url) => {
            info!("Remove register [{}]", &url);
            stat.remove(&url);
            map.write().remove(&url);
        }
    }

    let queue_len = util::get_block_queue_len(queue);
    // Deregister when the length of block queue is 0.
    if queue_len == 0 {
        return;
    }

    let max_block_height = util::get_max_block_height(queue);
    let min_block_height = util::get_min_block_height(queue);
    let min_push_height = match stat.values().min() {
        Some(height) => *height,
        None => 0,
    };
    assert!(min_push_height < max_block_height);

    for h in min_block_height..=min_push_height {
        info!(
            "Block status: queue len [{}], min height [{}], push height [{}], max height [{}]",
            queue_len, min_block_height, min_push_height, max_block_height
        );
        queue.write().remove(&h);
    }
}

fn rpc_handler<R: RegisterApi>(register: R) -> jsonrpc_core::IoHandler {
    let mut io = jsonrpc_core::IoHandler::default();
    io.extend_with(register.to_delegate());
    io
}

fn start_http_rpc_server(
    url: &str,
    io: jsonrpc_core::IoHandler,
) -> Result<jsonrpc_http_server::Server> {
    let server = jsonrpc_http_server::ServerBuilder::new(io)
        .threads(NUM_THREADS_FOR_REGISTERING)
        .rest_api(jsonrpc_http_server::RestApi::Unsecure)
        .cors(jsonrpc_http_server::DomainsValidation::AllowOnly(vec![
            jsonrpc_http_server::AccessControlAllowOrigin::Any,
        ]))
        .start_http(&url.parse()?)?;
    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_handle_existing_url() {
        let mut ctxt = Context::new(vec!["Balances FreeBalance1".into()], "1.0".into());
        ctxt.handle_existing_url("Balances FreeBalance2".into(), "1.0".into());
        assert_eq!(
            ctxt.prefixes,
            vec![
                "Balances FreeBalance1".to_string(),
                "Balances FreeBalance2".to_string()
            ]
        );
        assert_eq!(ctxt.version, "1.0".to_string());

        ctxt.handle_existing_url("Balances FreeBalance3".into(), "2.0".into());
        assert_eq!(ctxt.prefixes, vec!["Balances FreeBalance3".to_string(),]);
        assert_eq!(ctxt.version, "2.0".to_string());

        ctxt.handle_existing_url("Balances FreeBalance4".into(), "2.0".into());
        assert_eq!(
            ctxt.prefixes,
            vec![
                "Balances FreeBalance3".to_string(),
                "Balances FreeBalance4".to_string(),
            ]
        );
        assert_eq!(ctxt.version, "2.0".to_string());
    }
}
