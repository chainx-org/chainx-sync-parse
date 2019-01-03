#[macro_use]
extern crate log;

extern crate chainx_sub_parse;

use chainx_sub_parse::*;

const REDIS_SERVER_URL: &str = "redis://127.0.0.1";
const REGISTER_SERVER_URL: &str = "127.0.0.1:3030";

fn main() -> Result<()> {
    env_logger::init();

    let block_queue: BlockQueue = Arc::new(RwLock::new(BTreeMap::new()));
    println!("Block queue initial len: {}", block_queue.read().len());

    let transmit_thread = transmit::start(REGISTER_SERVER_URL.to_string(), block_queue.clone());

    let client = RedisClient::connect(REDIS_SERVER_URL)?;
    let subscribe_thread = client.start_subscription()?;

    let mut cur_block_height: u64 = 0;
    let mut stat = HashMap::new();

    while let Ok(key) = client.recv_key() {
        match client.query(&key) {
            Ok((height, value)) => {
                println!(
                    "block_height: {:?}, prefix+key: {:?}, value: {:?}",
                    height, key, value
                );
                if height == cur_block_height {
                    match RuntimeStorage::parse(&key, value) {
                        Ok((prefix, value)) => {
                            stat.insert(prefix, value);
                        }
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                    }
                    continue;
                }
                cur_block_height = height;
                let values: Vec<serde_json::Value> = stat.values().into_iter().cloned().collect();
                if let Some(_) = block_queue.write().insert(cur_block_height - 1, values) {
                    println!("Insert block failed");
                }
                stat.clear();

                let queue_len = block_queue.read().len();
                println!("BlockQueue len: {}", queue_len);
                let values = block_queue
                    .read()
                    .get(&(cur_block_height - 1))
                    .unwrap()
                    .clone();
                println!("Insert block: {:#?}", values);
            }
            Err(err) => {
                println!("{}", err);
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
