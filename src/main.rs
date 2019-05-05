#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::path::Path;
use std::thread::JoinHandle;

use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use serde_json::Value;
use structopt::StructOpt;

use chainx_sync_parse::*;

fn init_log_config(log_path: &Path) -> Result<()> {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {h({l})} - {m}\n",
        )))
        .build();
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {h({l})} - {m}\n",
        )))
        .build(log_path)?;

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

fn debug_sync_block_info(height: u64, key: &[u8], value: &[u8]) {
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
}

#[cfg(feature = "sync-log")]
fn sync_log(path: &str, start_height: u64, block_queue: &BlockQueue) -> Result<JoinHandle<()>> {
    let mut cur_block_height: u64 = start_height;
    let mut next_block_height: u64 = 0;
    let mut stat = HashMap::new();

    #[cfg(feature = "pgsql")]
    let pg_conn = establish_connection();

    let path = std::path::Path::new(path);
    assert!(path.is_file());
    let file = std::fs::File::open(path)?;

    let tail = Tail::new();
    let sync_service = tail.run(file)?;

    while let Ok((height, key, value)) = tail.recv_data() {
        debug_sync_block_info(height, &key, &value);

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

        insert_block_into_queue(block_queue, cur_block_height, &stat);
        #[cfg(feature = "pgsql")]
        insert_block_into_pgsql(&pg_conn, cur_block_height, &stat);

        stat.clear();
    }

    Ok(sync_service)
}

#[cfg(feature = "sync-redis")]
fn sync_redis(url: &str, _start_height: u64, block_queue: &BlockQueue) -> Result<JoinHandle<()>> {
    let mut cur_block_height: u64 = 0;
    let mut next_block_height: u64 = 0;
    let mut stat = HashMap::new();

    #[cfg(feature = "pgsql")]
    let pg_conn = establish_connection();

    let client = Redis::connect(url)?;
    let sync_service = client.start_subscription()?;

    while let Ok(key) = client.recv_key() {
        if let Ok((height, value)) = client.query(&key) {
            debug_sync_block_info(height, &key, &value);

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

            insert_block_into_queue(block_queue, cur_block_height, &stat);
            #[cfg(feature = "pgsql")]
            insert_block_into_pgsql(&pg_conn, cur_block_height, &stat);

            stat.clear();
        } else {
            error!("Redis query error");
            break;
        }
    }

    Ok(sync_service)
}

fn main() -> Result<()> {
    let cli = Cli::from_args();

    init_log_config(&cli.log_file_path)?;

    let block_queue: BlockQueue = BlockQueue::default();

    let register_server = RegisterService::new(block_queue.clone())
        .run(&format!("0.0.0.0:{}", cli.register_service_port))?;

    #[cfg(feature = "sync-log")]
    let sync_service = sync_log(&cli.sync_log_path, cli.start_height, &block_queue)?;
    #[cfg(feature = "sync-redis")]
    let sync_service = sync_redis(&cli.sync_redis_url, cli.start_height, &block_queue)?;

    sync_service
        .join()
        .expect("Couldn't join on the sync_service thread");

    register_server.wait();

    Ok(())
}
