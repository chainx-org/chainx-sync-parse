use std::fmt::Debug;
use std::thread;
use std::time::Duration;

use serde::de::DeserializeOwned;

use super::util;
use crate::{BlockQueue, Result};

const MSG_CHUNK_SIZE_LIMIT: usize = 10;

#[derive(PartialEq, Clone, Debug, Serialize)]
pub struct Message {
    height: u64,
    data: Vec<serde_json::Value>,
}

impl Message {
    /// Build a message with all json value that match the prefix successfully.
    pub fn build(block_queue: &BlockQueue, height: u64, prefixes: &[&str]) -> Option<Self> {
        if let Some(values) = block_queue.read().get(&height) {
            let mut data = vec![];
            values.iter().for_each(|value| {
                let need = util::get_value_prefix(value);
                prefixes.iter().for_each(|need| {
                    if *need == &util::get_value_prefix(value) {
                        data.push(value.clone());
                    }
                });
            });
            match data.is_empty() {
                false => Some(Self { height, data }),
                true => None,
            }
        } else {
            warn!("Cannot find the block whose height: {:?}", height);
            None
        }
    }

    /// Split the message into multiple messages according to `chunk_size`.
    pub fn split(self, chunk_size: usize) -> Vec<Self> {
        debug!("The message was split into multiple messages");
        self.data
            .chunks(chunk_size)
            .map(|value| Message {
                height: self.height,
                data: value.to_vec(),
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

#[derive(Debug, Deserialize)]
struct JsonResponse<T> {
    result: T,
}

#[derive(Clone)]
pub struct PushClient {
    url: String,
    /// The http rpc client for sending JSON-RPC request.
    client: reqwest::Client,
    /// The config of sending JSON-RPC request.
    config: Config,
}

impl PushClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            client: reqwest::Client::new(),
            config: Config::default(),
        }
    }

    pub fn post<T>(&self, body: &serde_json::Value) -> Result<T>
    where
        T: Debug + DeserializeOwned,
    {
        let resp: serde_json::Value = self
            .client
            .post(&self.url)
            .json(body)
            .send()?
            .json::<serde_json::Value>()?;
        let resp: JsonResponse<T> = serde_json::from_value(resp)?;
        Ok(resp.result)
    }

    pub fn post_message(&self, msg: &Message) -> Result<()> {
        let body: serde_json::Value = json!(msg);
        debug!("Send message request: {:?}", body);
        for i in 1..=self.config.retry_count {
            let ok = self.post::<String>(&body)?;
            debug!("Receive message response: {:?}", ok);
            if ok == "OK" {
                return Ok(());
            }
            warn!(
                "Send message request retry ({} / {})",
                i, self.config.retry_count
            );
            thread::sleep(self.config.retry_interval);
        }
        warn!(
            "Reach the limitation of retries, failed to send message: {:?}",
            msg
        );
        Err("Reach the limitation of retries".into())
    }

    pub fn post_big_message(&self, msg: Message) -> Result<()> {
        let messages = msg.split(MSG_CHUNK_SIZE_LIMIT);
        for msg in messages {
            if let Err(err) = self.post_message(&msg) {
                return Err(err);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! value {
        ($v:expr) => {
            serde_json::from_str::<serde_json::Value>($v).unwrap()
        };
    }

    macro_rules! values {
        ($v:expr) => {
            serde_json::from_str::<Vec<serde_json::Value>>($v).unwrap()
        };
    }

    #[test]
    fn test_build_message() {
        let queue = BlockQueue::default();
        queue.write().insert(
            0,
            values!(r#"[{"prefix":"aaa", "value":100}, {"prefix":"bbb", "value":100}, {"prefix":"ccc", "value":100}]"#)
        );
        assert_eq!(
            Message::build(&queue, 0, &["aaa", "bbb"]),
            Some(Message {
                height: 0,
                data: vec![
                    value!(r#"{"prefix":"aaa", "value":100}"#),
                    value!(r#"{"prefix":"bbb", "value":100}"#)
                ]
            })
        );

        queue.write().insert(
            1,
            values!(r#"[{"prefix":"aaa", "value":100}, {"prefix":"bbb", "value":200}, {"prefix":"ccc", "value":100}]"#)
        );
        assert_eq!(
            Message::build(&queue, 1, &["bbb", "ccc"]),
            Some(Message {
                height: 1,
                data: vec![
                    value!(r#"{"prefix":"bbb", "value":200}"#),
                    value!(r#"{"prefix":"ccc", "value":100}"#)
                ]
            })
        );

        queue.write().insert(
            2,
            values!(r#"[{"prefix":"aaa", "value":100}, {"prefix":"bbb", "value":200}, {"prefix":"ccc", "value":300}]"#)
        );
        assert_eq!(
            Message::build(&queue, 2, &["aaa", "ccc"]),
            Some(Message {
                height: 2,
                data: vec![
                    value!(r#"{"prefix":"aaa", "value":100}"#),
                    value!(r#"{"prefix":"ccc", "value":300}"#)
                ]
            })
        );
        assert_eq!(
            Message::build(&queue, 2, &["aaa", "ddd"]),
            Some(Message {
                height: 2,
                data: vec![value!(r#"{"prefix":"aaa", "value":100}"#),]
            })
        );
        assert_eq!(Message::build(&queue, 2, &["ddd"]), None);
    }

    #[test]
    fn test_split_message() {
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
