#[macro_use]
extern crate log;
extern crate env_logger;

extern crate chainx_sub_parse;

use env_logger::{Builder, Env};

use chainx_sub_parse::*;

const REDIS_SERVER_URL: &str = "redis://127.0.0.1";
const REGISTER_SERVER_URL: &str = "127.0.0.1:3030";

fn main() -> Result<()> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let block_queue: BlockQueue = Arc::new(RwLock::new(BTreeMap::new()));
    info!("Block queue initial len: {}", block_queue.read().len());

    let transmit_thread = transmit::start(REGISTER_SERVER_URL.to_string(), block_queue.clone());

    let client = RedisClient::connect(REDIS_SERVER_URL)?;
    let subscribe_thread = client.start_subscription()?;

    let mut cur_block_height: u64 = 0;
    let mut stat = HashMap::new();

    while let Ok(key) = client.recv_key() {
        match client.query(&key) {
            Ok((height, value)) => {
                info!(
                    "block_height: {:?}, prefix+key: {:?}, value: {:?}",
                    height,
                    ::std::str::from_utf8(&key).unwrap_or("Contains invalid UTF8"),
                    value
                );
                if height == cur_block_height {
                    match RuntimeStorage::parse(&key, value) {
                        Ok((prefix, value)) => {
                            stat.insert(prefix, value);
                        }
                        Err(e) => {
                            warn!("Runtime storage parse error: {}", e);
                            continue;
                        }
                    }
                    continue;
                }
                cur_block_height = height;
                let values: Vec<serde_json::Value> = stat.values().into_iter().cloned().collect();
                if let Some(_) = block_queue.write().insert(cur_block_height - 1, values) {
                    warn!("Failed to insert the block into block queue");
                }
                stat.clear();

                let queue_len = block_queue.read().len();
                info!("BlockQueue len: {:?}", queue_len);
                let values = block_queue
                    .read()
                    .get(&(cur_block_height - 1))
                    .unwrap()
                    .clone();
                info!("Insert block: {:#?}", values);
            }
            Err(err) => {
                warn!("Redis query error: {}", err);
                break;
            }
        }
    }

    subscribe_thread
        .join()
        .expect("Couldn't join on the subscribe thread")
        .unwrap_or_else(|e| println!("The detail of redis subscribe error: {:?}", e));

    transmit_thread
        .join()
        .expect("Couldn't join on the transmit thread");

    Ok(())
}
