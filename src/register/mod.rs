mod push;
mod util;

use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use jsonrpc_core::{IoHandler, Params};
use jsonrpc_http_server::{
    AccessControlAllowOrigin, DomainsValidation, RestApi, Server, ServerBuilder,
};
use parking_lot::{Mutex, RwLock};
use serde_json::Value;

use self::push::{Message, PushClient};
use crate::{BlockQueue, Result};

const NUM_THREADS_FOR_REGISTERING: usize = 4;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
struct Context {
    /// The prefixes of block storage that required by registrant.
    pub prefixes: Vec<String>,
    /// The representation that used to distinguish whether the storage info matches the requirements.
    pub version: String,
    /// The block height of the block that has been pushed.
    pub push_height: u64,
}

impl Context {
    pub fn new(prefixes: Vec<String>, version: String) -> Self {
        Self {
            prefixes,
            version,
            push_height: 0,
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

type PushData = (String, u64, bool);
type PushSender = Sender<PushData>;
type PushReceiver = Receiver<PushData>;

pub struct RegisterService {
    /// The block queue (BTreeMap: key - block height, value - json value).
    block_queue: BlockQueue,
    /// The map of register url and register context.
    map: RegisterMap,
}

impl RegisterService {
    pub fn run(url: &str, block_queue: BlockQueue) -> Result<Server> {
        let service = RegisterService::new(block_queue);
        let (tx, rx) = mpsc::channel();
        service.spawn_remove_block(rx);

        let mut io = IoHandler::new();
        let tx = Mutex::new(tx);
        service.register(&mut io, tx);

        start_http_rpc_server(url, io)
    }

    fn new(block_queue: BlockQueue) -> Self {
        Self {
            block_queue,
            map: Default::default(),
        }
    }

    fn register(self, io: &mut IoHandler, tx: Mutex<PushSender>) {
        io.add_method("register", move |params: Params| {
            let (prefix, url, version): (String, String, String) = params.parse().unwrap();
            let register_detail = format!(
                "url: {:?}, prefix: {:?}, version: {:?}",
                &url, &prefix, &version
            );
            self.map
                .write()
                .entry(url.clone())
                .and_modify(|ctxt| {
                    info!("Register existing [ {} ]", register_detail);
                    ctxt.lock()
                        .handle_existing_url(prefix.clone(), version.clone())
                })
                .or_insert_with(|| {
                    info!("Register new [ {} ]", register_detail);
                    let ctxt = Arc::new(Mutex::new(Context::new(vec![prefix], version)));
                    self.spawn_new_push(url, ctxt.clone(), tx.lock().clone());
                    ctxt
                });

            Ok(Value::String("OK".to_string()))
        });
    }

    fn spawn_new_push(&self, url: String, ctxt: RegisterContext, tx: PushSender) {
        let queue = self.block_queue.clone();
        let client = PushClient::new(url);
        let push_height = ctxt.lock().push_height;
        info!("Register: start push thread of url: [{}]", &client.url);

        thread::spawn(move || 'outer: loop {
            // Ensure that there is at least one block in the queue.
            if queue.read().len() <= 1 {
                thread::sleep(Duration::from_secs(1));
                continue;
            }
            let max_block_height = util::get_max_block_height(&queue);
            for h in push_height..max_block_height {
                if let Some(values) = queue.read().get(&h) {
                    let msg = Message::build(h, values, &ctxt.lock().prefixes);
                    if !msg.is_empty() && client.post_big_message(msg).is_err() {
                        tx.send((client.url.clone(), h, false))
                            .expect("Unable to send context");
                        // TODO: save abnormal register in the disk.
                        break 'outer;
                    } else {
                        tx.send((client.url.clone(), h, true))
                            .expect("Unable to send context");
                    }
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
                    Err(TryRecvError::Empty) => (),
                    Err(TryRecvError::Disconnected) => {
                        error!("Register: remove block thread terminated");
                        break;
                    }
                }
            }
        });
    }
}

fn remove_block_from_queue(queue: &BlockQueue, stat: &mut HashMap<String, u64>, map: &RegisterMap, data: PushData) {
    let (url, push_height, is_normal) = data;
    if is_normal {
        stat.entry(url)
            .and_modify(|height| *height = push_height)
            .or_insert(push_height);
    } else {
        error!("Register: [{}] 's push thread terminated", &url);
        stat.remove(&url);
        map.write().remove(&url);
    }

    let queue_len = util::get_block_queue_len(queue);
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

fn start_http_rpc_server(url: &str, io: IoHandler) -> Result<Server> {
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
