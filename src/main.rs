#[macro_use]
extern crate log;
extern crate chainx_sub_parse;

use std::u64;

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

    let client = RedisClient::connect(REDIS_SERVER_URL)?;
    let subscribe_thread = client.start_subscription()?;

    let mut cur_block_height: u64 = u64::max_value();

    while let Ok(key) = client.recv_key() {
        match client.query_value(key) {
            Ok((height, value)) => {
                if height == cur_block_height {
                    continue;
                }
                cur_block_height = height;

                specific_logic(cur_block_height, value);
            }
            Err(err) => {
                warn!("{}", err);
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

fn specific_logic(cur_block_height: u64, value: String) {
    println!(
        "cur_block_height: {:?}, value: {:?}",
        cur_block_height, value
    );
}
