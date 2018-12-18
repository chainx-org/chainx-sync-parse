#[macro_use]
extern crate log;
extern crate chainx_sub_parse;

use chainx_sub_parse::{
    get_runtime_modules_metadata, parse_metadata, MsQueue, RedisClient, Result,
};

const REDIS_SERVER_URL: &str = "redis://127.0.0.1";
const RPC_HTTP_URL: &str = "http://127.0.0.1:8081";

fn main() -> Result<()> {
    env_logger::init();

    // parse module metadata, create mapping table.
    //    let modules = get_runtime_modules_metadata(RPC_HTTP_URL)?;
    //    println!("Modules Metadata: {:#?}", modules);
    //    parse_metadata(modules)?;

    //    let msg_queue: MsQueue<serde_json::Value> = MsQueue::new();

    let client = RedisClient::connect(REDIS_SERVER_URL)?;
    let subscribe_thread = client.start_subscription()?;

    while let Ok(key) = client.recv_key() {
        match client.query_value(key) {
            Ok(value) => {
                println!("value: {:?}", value);
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
