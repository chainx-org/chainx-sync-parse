use std::time::Duration;

use jsonrpc_core::Result as RpcResult;
use jsonrpc_http_server::{
    AccessControlAllowOrigin, DomainsValidation, RestApi, Server, ServerBuilder,
};

use crate::{BlockQueue, Result};

const THREAD_POOL_NUM_THREADS: usize = 8;
const MSG_CHUNK_SIZE_LIMIT: usize = 10;

#[derive(PartialEq, Clone, Debug, Serialize)]
pub struct Message {
    height: u64,
    data: Vec<serde_json::Value>,
}

impl Message {
    pub fn new(height: u64) -> Self {
        Self {
            height,
            data: vec![],
        }
    }

    pub fn add(&mut self, value: serde_json::Value) {
        self.data.push(value);
    }

    /// Split the message into multiple messages according to `chunk_size`.
    pub fn split(self, chunk_size: usize) -> Vec<Self> {
        debug!("The message was split into multiple messages");
        let chunks = self
            .data
            .chunks(chunk_size)
            .map(|value| value.to_vec())
            .collect::<Vec<Vec<serde_json::Value>>>();
        chunks
            .into_iter()
            .map(|chunk| Message {
                height: self.height,
                data: chunk,
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    retry_count: u32,
    retry_interval: Duration,
}

impl Config {
    pub fn new(retry_count: u32, retry_interval: Duration) -> Self {
        Self {
            retry_count,
            retry_interval,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(3, Duration::new(3, 0))
    }
}

#[derive(Debug)]
pub struct PushClient {
    /// The http rpc client for sending JSON-RPC request.
    client: reqwest::Client,
    /// The block queue (BTreeMap: key - block height, value - json value).
    block_queue: BlockQueue,
}

impl PushClient {
    pub fn new(block_queue: BlockQueue) -> Self {
        Self {
            client: reqwest::Client::new(),
            block_queue,
        }
    }

    pub fn run(&self) -> Result<()> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(THREAD_POOL_NUM_THREADS)
            .build()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_split() {
        macro_rules! value {
            ($v:expr) => {
                serde_json::from_str::<serde_json::Value>($v).unwrap()
            };
        }

        let message = Message {
            height: 123,
            data: vec![
                value!("1"),
                value!("2"),
                value!("3"),
                value!("4"),
                value!("5"),
            ],
        };

        assert_eq!(
            vec![
                Message {
                    height: 123,
                    data: vec![value!("1"), value!("2")]
                },
                Message {
                    height: 123,
                    data: vec![value!("3"), value!("4")]
                },
                Message {
                    height: 123,
                    data: vec![value!("5")]
                },
            ],
            message.clone().split(2)
        );

        assert_eq!(vec![message.clone()], message.split(5));
    }
}
