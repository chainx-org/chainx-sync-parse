use redis::{IntoConnectionInfo, Value};

use std::sync::mpsc;
use std::thread;

use error::Result;

const REDIS_KEY_EVENT_NOTIFICATION: &str = "__keyevent@0__:zadd";

pub struct RedisClient {
    client: redis::Client,
    conn: redis::Connection,
    tx: mpsc::Sender<String>,
    pub rx: mpsc::Receiver<String>,
}

impl RedisClient {
    pub fn open<I: IntoConnectionInfo>(info: I) -> Result<Self> {
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

    pub fn query_value(&self, key: String) -> Result<String> {
        let key_score: Value = redis::cmd("ZREVRANGEBYSCORE")
            .arg(key)
            .arg("+inf")
            .arg("-inf")
            .arg("WITHSCORES")
            .arg("LIMIT")
            .arg(0)
            .arg(1)
            .query(&self.conn)?;
        let (key, score): (String, String) = redis::from_redis_value(&key_score)?;
        debug!("key = {:?}, score = {:?}", key, score);
        let val = redis::cmd("GET")
            .arg(format!("{}#{}", key, score))
            .query(&self.conn)?;
        let value = redis::from_redis_value(&val)?;
        Ok(value)
    }

    pub fn start_subscription(&self) -> thread::JoinHandle<Result<()>> {
        let tx = self.tx.clone();
        let mut sub_conn = self
            .client
            .get_connection()
            .expect("Create redis connection failed");

        thread::spawn(move || {
            let mut pubsub = sub_conn.as_pubsub();
            pubsub.subscribe(REDIS_KEY_EVENT_NOTIFICATION)?;

            loop {
                let msg = pubsub.get_message()?;
                match msg.get_channel_name() {
                    REDIS_KEY_EVENT_NOTIFICATION => {
                        let key = msg.get_payload::<String>()?;
                        tx.send(key)?;
                    }
                    _ => {
                        warn!("Wrong channel");
                        break;
                    }
                }
            }

            Ok(())
        })
    }
}
