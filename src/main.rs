#[macro_use]
extern crate log;
extern crate chainx_sub_parse;

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
    let block_queue: Arc<RwLock<BTreeMap<u64, serde_json::Value>>> =
        Arc::new(RwLock::new(BTreeMap::new()));

    let transmit = Client::new(REGISTER_SERVER_URL.to_string(), block_queue);
    let transmit_thread = transmit.start()?;

//    let client = RedisClient::connect(REDIS_SERVER_URL)?;
//    let subscribe_thread = client.start_subscription()?;
//
//    while let Ok(key) = client.recv_key() {
//        match client.query_value(key) {
//            Ok(value) => {
//                println!("value: {:?}", value);
//            }
//            Err(err) => {
//                warn!("{}", err);
//                break;
//            }
//        }
//    }
//
//    subscribe_thread
//        .join()
//        .expect("Couldn't join on the subscribe thread")
//        .unwrap_or_else(|e| println!("The detail of subscribe error: {:?}", e));

    transmit_thread
        .join()
        .expect("Couldn't join on the transmit thread")
        .unwrap_or_else(|e| println!("The detail of transmit error: {:?}", e));

    Ok(())
}
