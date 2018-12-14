extern crate chainx_sub_parse;

use chainx_sub_parse::{MsQueue, RedisClient, Result};

const REDIS_URL: &str = "redis://127.0.0.1";
//const RPC_HTTP_URL: &str = "http://127.0.0.1:8081";

fn main() {
    env_logger::init();

    // parse module metadata, create mapping table.
    //    let modules = parse::get_runtime_modules_metadata(RPC_HTTP_URL).unwrap();
    //    println!("Modules Metadata: {:#?}", modules);

    let msg_queue: MsQueue<serde_json::Value> = MsQueue::new();

    let client = RedisClient::open(REDIS_URL).expect("Create redis connection failed");
    let subscribe_thread = client.start_subscription();

    while let Ok(key) = client.rx.recv() {
        match client.query_value(key) {
            Ok(value) => println!("value: {:?}", value),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }

    subscribe_thread
        .join()
        .expect("Couldn't join on the subscribe thread")
        .unwrap_or_else(|e| println!("The detail of redis subscribe error: {:?}", e));
}
