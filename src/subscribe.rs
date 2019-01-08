use std::sync::mpsc;
use std::thread;

use crate::Result;

const REDIS_KEY_EVENT_NOTIFICATION: &str = "__keyevent@0__:zadd";

pub struct RedisClient {
    client: redis::Client,
    conn: redis::Connection,
    tx: mpsc::Sender<Vec<u8>>,
    rx: mpsc::Receiver<Vec<u8>>,
}

impl RedisClient {
    pub fn connect<I: redis::IntoConnectionInfo>(info: I) -> Result<Self> {
        let client = redis::Client::open(info)?;
        let conn = client.get_connection()?;
        let (tx, rx) = mpsc::channel();
        Ok(Self {
            client,
            conn,
            tx,
            rx,
        })
    }

    pub fn query(&self, key: &[u8]) -> Result<(u64, Vec<u8>)> {
        let (key, score): (Vec<u8>, u64) = self.query_key_and_score(key)?;
        let value = self.query_value(&key)?;
        Ok((score, value))
    }

    #[rustfmt::skip]
    fn query_key_and_score(&self, key: &[u8]) -> Result<(Vec<u8>, u64)> {
        let (key, score): (Vec<u8>, u64) = redis::cmd("ZREVRANGEBYSCORE")
            .arg(key)
            .arg("+inf").arg("-inf")
            .arg("WITHSCORES")
            .arg("LIMIT").arg(0).arg(1)
            .query(&self.conn)?;
        debug!(
            "key: {:?}, score: {:?}",
            ::std::str::from_utf8(&key).unwrap_or("Contains invalid UTF8"), score
        );
        Ok((key, score))
    }

    fn query_value(&self, key: &[u8]) -> Result<Vec<u8>> {
        let value: Vec<u8> = redis::cmd("GET").arg(key).query(&self.conn)?;
        debug!("value: {:?}", value);
        Ok(value)
    }

    pub fn recv_key(&self) -> Result<Vec<u8>> {
        Ok(self.rx.recv()?)
    }

    pub fn start_subscription(&self) -> Result<thread::JoinHandle<()>> {
        let tx = self.tx.clone();
        let mut sub_conn = self.client.get_connection()?;

        let thread = thread::spawn(move || {
            let mut pubsub = sub_conn.as_pubsub();
            pubsub
                .subscribe(REDIS_KEY_EVENT_NOTIFICATION)
                .unwrap_or_else(|err| warn!("Subscribe error: {:?}", err));

            while let Ok(msg) = pubsub.get_message() {
                if msg.get_channel_name() == REDIS_KEY_EVENT_NOTIFICATION {
                    let key: Vec<u8> = match msg.get_payload() {
                        Ok(key) => key,
                        Err(err) => {
                            warn!("Msg get payload error: {:?}", err);
                            break;
                        }
                    };
                    if let Err(err) = tx.send(key) {
                        warn!("Send error: {:?}", err);
                        break;
                    }
                } else {
                    warn!("Wrong channel");
                    break;
                }
            }
            warn!("Pubsub get msg error, exit subscription loop");
        });

        Ok(thread)
    }
}
