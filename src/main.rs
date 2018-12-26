#[macro_use]
extern crate log;
extern crate chainx_sub_parse;

use std::u64;

use chainx_sub_parse::*;

const REDIS_SERVER_URL: &str = "redis://127.0.0.1";
//const RPC_HTTP_URL: &str = "http://127.0.0.1:8081";
const REGISTER_SERVER_URL: &str = "127.0.0.1:3030";

fn main() -> Result<()> {
    env_logger::init();

    // parse module metadata, create mapping table.
    //        let runtime_metadata = get_runtime_metadata(RPC_HTTP_URL)?;
    //        println!("Modules Metadata: {:#?}", modules);
    //        parse_metadata(runtime_metadata)?;

    let block_queue: BlockQueue = Arc::new(RwLock::new(BTreeMap::new()));

    let transmit = Client::new(REGISTER_SERVER_URL.to_string(), block_queue);
    let transmit_thread = transmit.start()?;

    let client = RedisClient::connect(REDIS_SERVER_URL)?;
    let subscribe_thread = client.start_subscription()?;

    let mut cur_block_height: u64 = u64::max_value();

    while let Ok(key) = client.recv_key() {
        match client.query(key) {
            Ok((height, key, value)) => {
                if height == cur_block_height {
                    continue;
                }
                cur_block_height = height;
                // specific logic
                println!(
                    "cur_block_height: {:?}, key: {:?}, value: {:?}",
                    cur_block_height, key, value
                );
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
        .expect("Couldn't join on the transmit thread")
        .unwrap_or_else(|e| println!("The detail of transmit error: {:?}", e));

    Ok(())
}
