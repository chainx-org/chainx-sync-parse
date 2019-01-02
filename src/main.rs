#[macro_use]
extern crate log;

extern crate chainx_sub_parse;

use chainx_sub_parse::*;

const REDIS_SERVER_URL: &str = "redis://127.0.0.1";

fn main() -> Result<()> {
    env_logger::init();

    let block_queue: BlockQueue = Arc::new(RwLock::new(BTreeMap::new()));
    println!("Block queue initial len: {}", block_queue.read().len());

    let client = RedisClient::connect(REDIS_SERVER_URL)?;
    let subscribe_thread = client.start_subscription()?;

    let mut cur_block_height: u64 = u64::max_value();

    while let Ok(key) = client.recv_key() {
        match client.query(&key) {
            Ok((height, value)) => {
                //                if height == cur_block_height {
                //                    continue;
                //                }
                //                cur_block_height = height;
                println!(
                    "block_height: {:?}, prefix+key: {:?}, value: {:?}",
                    height, key, value
                );
                let block_value = match RuntimeStorage::parse(&key, value) {
                    Ok(value) => value,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                println!("json: {:?}", block_value);
                //                if let Some(_) = block_queue.write().insert(cur_block_height, block_value) {
                //                    println!("Insert block failed");
                //                }
                //                let queue_len = block_queue.read().len();
                //                println!("queue len: {}", queue_len);
                //                let value = block_queue.read().get(&cur_block_height).unwrap().clone();
                //                println!("Insert block: {:#?}", value);
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

    Ok(())
}
