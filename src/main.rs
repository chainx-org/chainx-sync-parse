#[macro_use]
extern crate log;
extern crate log4rs;

extern crate chainx_sub_parse;

use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

use chainx_sub_parse::*;

const REDIS_SERVER_URL: &str = "redis://127.0.0.1";
const REGISTER_SERVER_URL: &str = "127.0.0.1:3030";
const LOG_FILE_PATH: &str = "log/output.log";

fn main() -> Result<()> {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {h({l})} - {m}\n",
        )))
        .build();
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {h({l})} - {m}\n",
        )))
        .build(LOG_FILE_PATH)?;

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appenders(vec!["console", "file"])
                .build(LevelFilter::Info),
        )
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    let block_queue: BlockQueue = Arc::new(RwLock::new(BTreeMap::new()));
    debug!("BlockQueue len: {}", block_queue.read().len());

    let register_service_thread = RegisterService::run(REGISTER_SERVER_URL, block_queue.clone())?;

    let client = RedisClient::connect(REDIS_SERVER_URL)?;
    let subscribe_thread = client.start_subscription()?;

    let mut cur_block_height: u64 = 0;
    let mut stat = HashMap::new();

    while let Ok(key) = client.recv_key() {
        if let Ok((height, value)) = client.query(&key) {
            debug!(
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
                    Err(_) => continue,
                }
                continue;
            }
            cur_block_height = height;
            let values: Vec<serde_json::Value> = stat.values().cloned().collect();
            info!(
                "Current block height: {:?}, block info: {:?}",
                cur_block_height - 1,
                serde_json::Value::Array(values.clone()).to_string()
            );
            if block_queue
                .write()
                .insert(cur_block_height - 1, values)
                .is_some()
            {
                warn!("Failed to insert the block into block queue");
            }
            stat.clear();

        // let queue_len = block_queue.read().len();
        // debug!("BlockQueue len: {:?}", queue_len);
        } else {
            warn!("Redis query error");
            break;
        }
    }

    subscribe_thread
        .join()
        .expect("Couldn't join on the subscribe thread");

    register_service_thread
        .join()
        .expect("Couldn't join on the transmit thread");

    Ok(())
}
