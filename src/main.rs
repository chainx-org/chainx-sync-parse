#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::thread::JoinHandle;

use chainx_sync_parse::*;
use log::LevelFilter;
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            policy::{
                self,
                compound::{roll, trigger},
            },
            RollingFileAppender,
        },
    },
    config,
    encode::pattern::PatternEncoder,
};
use serde_json::Value;
use structopt::StructOpt;

const MB_SIZE: u64 = 1024 * 1024; // 1 MB

fn init_log_with_config(config: &CliConfig) -> Result<()> {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {h({l})} - {m}\n",
        )))
        .build();

    let trigger = trigger::size::SizeTrigger::new(config.parse_roll_size * MB_SIZE);
    let roll_pattern = format!("{}.{{}}.gz", config.parse_log_path.to_str().unwrap());
    let roll = roll::fixed_window::FixedWindowRoller::builder()
        .build(roll_pattern.as_str(), config.parse_roll_count)
        .expect("Building fixed window roller should't be fail");
    let policy = policy::compound::CompoundPolicy::new(Box::new(trigger), Box::new(roll));
    let parse_roll_file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {h({l})} - {m}\n",
        )))
        .build(&config.parse_log_path, Box::new(policy))?;

    let log_config = config::Config::builder()
        .appender(config::Appender::builder().build("console", Box::new(console)))
        .appender(config::Appender::builder().build("parse_roll", Box::new(parse_roll_file)))
        .logger(
            config::Logger::builder()
                .appender("parse_roll")
                .build("parse", LevelFilter::Info),
        )
        .build(
            config::Root::builder()
                .appender("console")
                .build(LevelFilter::Info),
        )
        .expect("Construct log config failure");

    log4rs::init_config(log_config).expect("Initializing log config shouldn't be fail");
    Ok(())
}

fn version_info() {
    info!(target: "parse", "============================================================");
    info!(
        target: "parse",
        "Release Version:   {}",
        option_env!("CARGO_PKG_VERSION").unwrap_or("Unknown")
    );
    info!(
        target: "parse",
        "Git Commit Hash:   {}",
        option_env!("BUILD_GIT_HASH").unwrap_or("Unknown")
    );
    info!(
        target: "parse",
        "Git Commit Branch: {}",
        option_env!("BUILD_GIT_BRANCH").unwrap_or("Unknown")
    );
    info!(
        target: "parse",
        "Rust Version:      {}",
        option_env!("BUILD_RUSTC_VERSION").unwrap_or("Unknown")
    );
    info!(target: "parse", "============================================================");
}

fn insert_block_into_queue(queue: &BlockQueue, h: u64, stat: &HashMap<Vec<u8>, Value>) {
    let values: Vec<Value> = stat.values().cloned().collect();
    if queue.write().insert(h, values.clone()).is_none() {
        info!(target: "parse", "Insert new block #{} into block queue successfully", h);
        info!(target: "parse", "Block #{}: {:?}", h, Value::Array(values).to_string());
    } else {
        info!(target: "parse", "Insert updated block #{} into block queue successfully", h);
        info!(target: "parse", "Block #{}: {:?}", h, Value::Array(values).to_string());
    }
}

fn debug_sync_block_info(height: u64, key: &[u8], value: &[u8]) {
    // for debug
    if let Ok(prefix_key) = ::std::str::from_utf8(&key) {
        debug!(
            target: "parse",
            "Block info: block_height [{:?}], prefix_key [{:?}], value [{:?}]",
            height, prefix_key, value,
        );
    } else {
        debug!(
            target: "parse",
            "Block info: block_height [{:?}], prefix_key [{:?}], value [{:?}]",
            height, key, value,
        );
    }
}

#[cfg(feature = "sync-log")]
fn sync_log(config: &CliConfig, queue: &BlockQueue) -> Result<JoinHandle<()>> {
    assert!(
        config.start_height < config.stop_height,
        "Invalid block height range"
    );
    info!(target: "parse", "Scanned block height range, [start: {}, stop: {})", config.start_height, config.stop_height);

    #[cfg(feature = "pgsql")]
    let pg_conn = establish_connection();

    let tail = Tail::new();
    let sync_service = tail.run(config)?;

    let mut stat = HashMap::new();
    let mut next_block_height: u64 = config.start_height;

    while let Ok((height, key, value)) = tail.recv_data() {
        debug_sync_block_info(height, &key, &value);

        // handling sync block fallback
        if height < next_block_height {
            insert_block_into_queue(queue, next_block_height, &stat);
            #[cfg(feature = "pgsql")]
            insert_block_into_pgsql(&pg_conn, next_block_height, &stat);
            next_block_height = height;
            stat.clear();
        }

        // collect all data of the block with the same height
        if height == next_block_height {
            match RuntimeStorage::parse(&key, value) {
                Ok((prefix, value)) => {
                    let mut prefix = prefix.as_bytes().to_vec();
                    prefix.extend_from_slice(&key);
                    stat.insert(prefix, value);
                }
                Err(_) => continue,
            }
        } else {
            // when height > nex_block_height
            // Insert a complete block into queue.
            // Example: Once a block1 (height = 1) is received,
            // it means that the block0 (height = 0) has been synchronized and parsed.
            assert!(height >= 1);
            let insert_height = height - 1;
            insert_block_into_queue(queue, insert_height, &stat);
            #[cfg(feature = "pgsql")]
            insert_block_into_pgsql(&pg_conn, insert_height, &stat);
            next_block_height = height;
            stat.clear();
        }
    }

    Ok(sync_service)
}

#[cfg(feature = "sync-redis")]
fn sync_redis(config: &CliConfig, queue: &BlockQueue) -> Result<JoinHandle<()>> {
    let mut cur_block_height: u64 = 0;
    let mut next_block_height: u64 = 0;
    let mut stat = HashMap::new();

    #[cfg(feature = "pgsql")]
    let pg_conn = establish_connection();

    let client = Redis::connect(config.sync_redis_url.as_str())?;
    info!(target: "parse", "Connect redis [{}] successfully", &config.sync_redis_url);
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
            assert!(height >= 1);
            next_block_height = height;
            cur_block_height = height - 1;

            // Insert a complete block.
            // Example: Once a block1 (height = 1) is received,
            // it means that the block0 (height = 0) has been synchronized and parsed.
            insert_block_into_queue(queue, cur_block_height, &stat);
            #[cfg(feature = "pgsql")]
            insert_block_into_pgsql(&pg_conn, cur_block_height, &stat);

            stat.clear();
        } else {
            error!(target: "parse", "Redis query error");
            break;
        }
    }

    Ok(sync_service)
}

fn main() -> Result<()> {
    let config = CliConfig::from_args();
    init_log_with_config(&config)?;
    version_info();
    info!(
        target: "parse",
        "Parse log [path: {:?}, roll size: {:?}MB, roll count: {:?}]",
        config.parse_log_path, config.parse_roll_size, config.parse_roll_count
    );
    /*
    info!(
        target: "parse",
        "Msgbus log [path: {:?}, roll size: {:?}MB, roll count: {:?}]",
        config.msgbus_log_path, config.msgbus_roll_size, config.msgbus_roll_count
    );
    */

    let block_queue: BlockQueue = BlockQueue::default();

    let register_server = RegisterService::new(block_queue.clone())
        .run(&format!("0.0.0.0:{}", config.register_service_port))?;

    #[cfg(feature = "sync-log")]
    let sync_service = sync_log(&config, &block_queue)?;
    #[cfg(feature = "sync-redis")]
    let sync_service = sync_redis(&config, &block_queue)?;

    sync_service
        .join()
        .expect("Couldn't join on the sync_service thread");

    register_server.wait();

    Ok(())
}
