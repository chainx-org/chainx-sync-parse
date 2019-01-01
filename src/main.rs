#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate chainx_sub_parse;

use chainx_sub_parse::*;

const REDIS_SERVER_URL: &str = "redis://127.0.0.1";
//const RPC_HTTP_URL: &str = "http://127.0.0.1:8081";

fn main() -> Result<()> {
    env_logger::init();

    // parse module metadata, create mapping table.
    //        let runtime_metadata = get_runtime_metadata(RPC_HTTP_URL)?;
    //        println!("Modules Metadata: {:#?}", modules);
    //        parse_metadata(runtime_metadata)?;

    let block_queue: BlockQueue = Arc::new(RwLock::new(BTreeMap::new()));
    println!("Block queue initial len: {}", block_queue.read().len());

    let client = RedisClient::connect(REDIS_SERVER_URL)?;
    let subscribe_thread = client.start_subscription()?;

    let mut cur_block_height: u64 = u64::max_value();

    while let Ok(key) = client.recv_key() {
        match client.query(&key) {
            Ok((height, value)) => {
                if height == cur_block_height {
                    continue;
                }
                cur_block_height = height;

                println!(
                    "cur_block_height: {:?}, prefix+key: {:?}, value: {:?}",
                    height, key, value
                );
                let block_value = RuntimeStorage::parse(&key, value)?;
                if let Some(_) = block_queue.write().insert(cur_block_height, block_value) {
                    println!("Insert block failed");
                }
                let queue_len = block_queue.read().len();
                println!("queue len: {}", queue_len);
                if queue_len % 10 == 0 {
                    let values: Vec<serde_json::Value> = block_queue.read().values().cloned().collect();
                    println!("queue: {:?}", values);
                }
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
