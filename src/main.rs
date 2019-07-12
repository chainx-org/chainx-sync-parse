#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::thread::JoinHandle;

use chainx_sync_parse::*;

fn main() -> Result<()> {
    let config = cli::init();
    logger::init(&config)?;

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

#[cfg(feature = "sync-log")]
fn sync_log(config: &CliConfig, queue: &BlockQueue) -> Result<JoinHandle<()>> {
    assert!(
        config.start_height < config.stop_height,
        "Invalid block height range"
    );
    info!(
        "Scanned block height range, [start: {}, stop: {})",
        config.start_height, config.stop_height
    );

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
    info!("Connect redis [{}] successfully", &config.sync_redis_url);
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
            error!("Redis query error");
            break;
        }
    }

    Ok(sync_service)
}

fn insert_block_into_queue(queue: &BlockQueue, h: u64, stat: &HashMap<Vec<u8>, serde_json::Value>) {
    let values = stat.values().cloned().collect::<Vec<_>>();
    if queue.write().insert(h, values.clone()).is_none() {
        info!("Insert new block #{} into block queue successfully", h);
        info!(
            "Block #{}: {:?}",
            h,
            serde_json::Value::Array(values).to_string()
        );
    } else {
        info!("Insert updated block #{} into block queue successfully", h);
        info!(
            "Block #{}: {:?}",
            h,
            serde_json::Value::Array(values).to_string()
        );
    }
}

fn debug_sync_block_info(height: u64, key: &[u8], value: &[u8]) {
    // for debug
    if let Ok(prefix_key) = ::std::str::from_utf8(&key) {
        debug!(
            "Block info: block_height [{:?}], prefix_key [{:?}], value [{:?}]",
            height, prefix_key, value,
        );
    } else {
        debug!(
            "Block info: block_height [{:?}], prefix_key [{:?}], value [{:?}]",
            height, key, value,
        );
    }
}
