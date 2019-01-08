extern crate env_logger;
#[macro_use]
extern crate log;

extern crate chainx_sub_parse;

use env_logger::Builder;
use log::LevelFilter;

use chainx_sub_parse::*;

const REDIS_SERVER_URL: &str = "redis://127.0.0.1";

fn main() -> Result<()> {
    Builder::new()
        .default_format()
        .default_format_module_path(false)
        .filter_level(LevelFilter::Info)
        .init();

    let block_queue: BlockQueue = Arc::new(RwLock::new(BTreeMap::new()));
    info!("BlockQueue len: {}", block_queue.read().len());

    let client = RedisClient::connect(REDIS_SERVER_URL)?;
    let subscribe_thread = client.start_subscription()?;

    let mut cur_block_height: u64 = 0;
    let mut stat = HashMap::new();

    while let Ok(key) = client.recv_key() {
        if let Ok((height, value)) = client.query(&key) {
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
                    Err(err) => {
                        warn!("Runtime storage parse error: {}", err);
                        continue;
                    }
                }
                continue;
            }
            cur_block_height = height;
            let values: Vec<serde_json::Value> = stat.values().cloned().collect();
            if block_queue.write().insert(cur_block_height - 1, values).is_some() {
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
        } else {
            warn!("Redis query error");
            break;
        }
    }

    subscribe_thread
        .join()
        .expect("Couldn't join on the subscribe thread");

    Ok(())
}
