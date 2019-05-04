mod push;
mod rpc;
mod util;

use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use parking_lot::{Mutex, RwLock};
use semver::Version;

use self::push::{Message, PushClient};
use self::rpc::RegisterApi;
use crate::{BlockQueue, Result};

#[derive(PartialEq, Clone, Debug)]
struct Context {
    /// The prefixes of block storage that required by registrant.
    pub prefixes: HashSet<String>,
    /// The representation that used to distinguish whether the storage info matches the requirements.
    pub version: Version,
    /// The block height of the block that has been pushed.
    pub push_height: u64,
    /// The flag the represents whether registrant deregister.
    pub deregister: bool,
}

impl Context {
    pub fn new(prefixes: Vec<String>, version: Version) -> Self {
        Self {
            prefixes: prefixes.iter().cloned().collect(),
            version,
            push_height: 0,
            deregister: false,
        }
    }

    /// Update version and prefixes of the context.
    pub fn update_prefixes(&mut self, prefixes: Vec<String>, version: Version) {
        if version > self.version {
            self.version = version;
            self.prefixes.clear();
            self.prefixes.extend(prefixes);
            info!(
                "New version: [{}], Updated prefixes: [{:?}]",
                &self.version, &self.prefixes
            );
        } else {
            self.prefixes.extend(prefixes);
            info!("Updated prefixes: [{:?}]", &self.prefixes);
        }
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
    Deregister(String),
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
        let client = PushClient::new(url);
        info!("Register: start push thread of url: [{}]", &client.url);

        thread::spawn(move || 'outer: loop {
            if ctxt.lock().deregister {
                tx.send(NotifyData::Deregister(client.url.clone()))
                    .expect("Unable to send context");
                info!(
                    "Deregister: [{}] 's push thread will terminate",
                    &client.url
                );
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
                    warn!(
                        "Post abnormal: [{}] 's push thread will terminate",
                        &client.url
                    );
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
            info!("Abnormal, remove register [{}]", &url);
            stat.remove(&url);
            map.write().remove(&url);
            return;
        }
        NotifyData::Deregister(url) => {
            info!("Deregister, remove register [{}]", &url);
            stat.remove(&url);
            map.write().remove(&url);
            return;
        }
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
    fn test_context_update_prefixes() {
        let mut ctxt = Context::new(
            vec!["Balances FreeBalance1".into()],
            Version::parse("1.0.0").unwrap(),
        );
        ctxt.update_prefixes(
            vec!["Balances FreeBalance2".into()],
            Version::parse("1.0.0").unwrap(),
        );
        assert!(ctxt.prefixes.contains("Balances FreeBalance1"));
        assert!(ctxt.prefixes.contains("Balances FreeBalance2"));
        assert_eq!(ctxt.version, Version::new(1, 0, 0));

        ctxt.update_prefixes(
            vec!["Balances FreeBalance3".into()],
            Version::parse("1.1.0").unwrap(),
        );
        assert_eq!(ctxt.prefixes.contains("Balances FreeBalance1"), false);
        assert_eq!(ctxt.prefixes.contains("Balances FreeBalance2"), false);
        assert_eq!(ctxt.prefixes.contains("Balances FreeBalance3"), true);
        assert_eq!(ctxt.version, Version::new(1, 1, 0));

        ctxt.update_prefixes(
            vec!["Balances FreeBalance4".into()],
            Version::parse("1.1.0").unwrap(),
        );
        assert_eq!(ctxt.prefixes.contains("Balances FreeBalance1"), false);
        assert_eq!(ctxt.prefixes.contains("Balances FreeBalance2"), false);
        assert_eq!(ctxt.prefixes.contains("Balances FreeBalance3"), true);
        assert_eq!(ctxt.prefixes.contains("Balances FreeBalance4"), true);
        assert_eq!(ctxt.version, Version::new(1, 1, 0));
    }
}
