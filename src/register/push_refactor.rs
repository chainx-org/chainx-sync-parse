use std::fmt::Debug;
use std::thread;
use std::time::Duration;

use serde::de::DeserializeOwned;

use crate::Result;

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

#[derive(Debug, Deserialize)]
struct JsonResponse<T> {
    result: T,
}

#[derive(Clone)]
pub struct PushClient {
    /// The http rpc client for sending JSON-RPC request.
    client: reqwest::Client,
    /// The config of sending JSON-RPC request.
    config: Config,
}

impl PushClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            config: Config::default(),
        }
    }

    pub fn request<T>(&self, url: &str, body: &serde_json::Value) -> Result<T>
    where
        T: Debug + DeserializeOwned,
    {
        let resp: serde_json::Value = self
            .client
            .post(url)
            .json(body)
            .send()?
            .json::<serde_json::Value>()?;
        let resp: JsonResponse<T> = serde_json::from_value(resp)?;
        Ok(resp.result)
    }

    pub fn request_with_config(&self, url: &str, msg: Message) -> Result<()> {
        let body = json!(msg);
        debug!("Send message request: {:?}", body);
        for i in 1..=self.config.retry_count {
            let ok = self.request::<String>(url, &body)?;
            info!("Receive message response: {:?}", ok);
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
