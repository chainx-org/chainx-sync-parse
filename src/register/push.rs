use std::fmt::Debug;
use std::thread;
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde_json::Value;

use super::util;
use crate::Result;

const MSG_CHUNK_SIZE_LIMIT: usize = 10;

#[derive(PartialEq, Clone, Debug, Serialize)]
pub struct Message {
    height: u64,
    data: Vec<Value>,
}

impl Message {
    /// Build a message with all json value that match the prefix successfully.
    /// The data of message may be empty, and empty message don't need to be pushed.
    pub fn build(height: u64, values: &[Value], prefixes: &[String]) -> Self {
        let mut data = vec![];
        values.iter().for_each(|value| {
            let prefix = util::get_value_prefix(&value);
            prefixes.iter().for_each(|need| {
                if need == &prefix {
                    data.push(value.clone());
                }
            });
        });
        Self { height, data }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Split the message into multiple messages according to `chunk_size`.
    pub fn split(self, chunk_size: usize) -> Vec<Self> {
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
    pub url: String,
    client: reqwest::Client,
    config: Config,
}

impl PushClient {
    pub fn new(url: String) -> Self {
        Self {
            url,
            client: reqwest::Client::new(),
            config: Config::default(),
        }
    }

    pub fn post_big_message(&self, msg: Message) -> Result<()> {
        info!("Post message: {:?}", &msg);
        let messages = msg.split(MSG_CHUNK_SIZE_LIMIT);
        if messages.len() != 1 {
            info!("The message was split into {} messages", messages.len());
        }
        for msg in messages {
            if let Err(err) = self.post_message(&msg) {
                error!("Post error: {}, msg: {:?}", err, msg);
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn post_message(&self, msg: &Message) -> Result<()> {
        let body: Value = json!(msg);
        debug!("Send message request: {:?}", body);
        for i in 1..=self.config.retry_count {
            let ok = self.post::<String>(&body).unwrap_or_default();
            if ok == "OK" {
                return Ok(());
            }
            warn!("Receive message response: {:?}", ok);
            warn!(
                "Send message request retry ({} / {})",
                i, self.config.retry_count
            );
            thread::sleep(self.config.retry_interval);
        }
        error!("Reach the limitation of retries");
        Err("Reach the limitation of retries".into())
    }

    fn post<T>(&self, body: &Value) -> Result<T>
    where
        T: Debug + DeserializeOwned,
    {
        let resp = self
            .client
            .post(&self.url)
            .json(body)
            .send()?
            .json::<Value>()?;
        let resp: JsonResponse<T> = serde_json::from_value(resp)?;
        Ok(resp.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! value {
        ($v:expr) => {
            serde_json::from_str::<Value>($v).unwrap()
        };
    }

    macro_rules! values {
        ($v:expr) => {
            serde_json::from_str::<Vec<Value>>($v).unwrap()
        };
    }

    #[test]
    fn test_message_build() {
        let values0 = values!(
            r#"[
            {"prefix":"aaa", "value":100},
            {"prefix":"bbb", "value":100},
            {"prefix":"ccc", "value":100}
        ]"#
        );
        assert_eq!(
            Message::build(0, &values0, &["aaa".into(), "bbb".into()]),
            Message {
                height: 0,
                data: vec![
                    value!(r#"{"prefix":"aaa", "value":100}"#),
                    value!(r#"{"prefix":"bbb", "value":100}"#)
                ]
            }
        );

        let values1 = values!(
            r#"[
            {"prefix":"aaa", "value":100},
            {"prefix":"bbb", "value":200},
            {"prefix":"ccc", "value":100}
        ]"#
        );
        assert_eq!(
            Message::build(1, &values1, &["bbb".into(), "ccc".into()]),
            Message {
                height: 1,
                data: vec![
                    value!(r#"{"prefix":"bbb", "value":200}"#),
                    value!(r#"{"prefix":"ccc", "value":100}"#)
                ]
            }
        );

        let values2 = values!(
            r#"[
            {"prefix":"aaa", "value":100},
            {"prefix":"bbb", "value":200},
            {"prefix":"ccc", "value":300}
        ]"#
        );
        assert_eq!(
            Message::build(2, &values2, &["aaa".into(), "ccc".into()]),
            Message {
                height: 2,
                data: vec![
                    value!(r#"{"prefix":"aaa", "value":100}"#),
                    value!(r#"{"prefix":"ccc", "value":300}"#)
                ]
            }
        );
        assert_eq!(
            Message::build(2, &values2, &["aaa".into(), "ddd".into()]),
            Message {
                height: 2,
                data: vec![value!(r#"{"prefix":"aaa", "value":100}"#),]
            }
        );
        assert_eq!(
            Message::build(2, &values2, &["ddd".into()]),
            Message {
                height: 2,
                data: vec![]
            }
        );
    }

    #[test]
    fn test_message_split() {
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
