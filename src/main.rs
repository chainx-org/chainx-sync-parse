use std::collections::HashMap;

use log::{debug, error, info, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use serde_json::Value;

use chainx_sub_parse::*;

const REDIS_SERVER_URL: &str = "redis://127.0.0.1";
const REGISTER_SERVER_URL: &str = "0.0.0.0:3030";
const LOG_FILE_PATH: &str = "log/output.log";

fn init_log_config() -> Result<()> {
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
        .expect("Construct log config failure");

    log4rs::init_config(config).expect("Initialize log config failure");
    Ok(())
}

fn insert_block_into_queue(queue: &BlockQueue, h: u64, stat: &HashMap<Vec<u8>, Value>) {
    let values: Vec<Value> = stat.values().cloned().collect();
    if queue.write().insert(h, values.clone()).is_none() {
        info!("Insert the block #{} into block queue successfully", h);
        info!("block info: {:?}", Value::Array(values).to_string());
    }
}

fn main() -> Result<()> {
    init_log_config()?;

    let block_queue: BlockQueue = BlockQueue::default();
    #[cfg(feature = "pgsql")]
    let pg_conn = establish_connection();

    let register_server = RegisterService::new(block_queue.clone()).run(REGISTER_SERVER_URL)?;
    let client = RedisClient::connect(REDIS_SERVER_URL)?;
    let subscribe_service = client.start_subscription()?;

    let mut next_block_height: u64 = 0;
    let mut cur_block_height: u64 = 0;
    let mut stat = HashMap::new();

    while let Ok(key) = client.recv_key() {
        if let Ok((height, value)) = client.query(&key) {
            // for debug
            if let Ok(prefix_key) = ::std::str::from_utf8(&key) {
                debug!(
                    "block_height: {:?}, prefix+key: {:?}, value: {:?}",
                    height, prefix_key, value
                );
            } else {
                debug!(
                    "block_height: {:?}, prefix+key: Invalid UTF8 (hex: {:?}), value: {:?}",
                    height, key, value
                );
            }

            if height < cur_block_height {
                next_block_height = height;
                stat.clear();
            }

            if height == next_block_height {
                match RuntimeStorage::parse(&key, value) {
                    Ok((prefix, value)) => {
                        let mut prefix = prefix.as_bytes().to_vec();
                        prefix.extend_from_slice(&key);
                        stat.insert(prefix, value);
                    }
                    Err(_) => continue,
                }
                continue;
            }

            // when height > nex_block_height
            next_block_height = height;
            cur_block_height = next_block_height - 1;

            insert_block_into_queue(&block_queue, cur_block_height, &stat);
            #[cfg(feature = "pgsql")]
            insert_block_into_pgsql(&pg_conn, cur_block_height, &stat);

            stat.clear();
        } else {
            error!("Redis query error");
            break;
        }
    }

    subscribe_service
        .join()
        .expect("Couldn't join on the subscribe thread");

    register_server.wait();

    Ok(())
}
