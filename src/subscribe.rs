use std::sync::mpsc;
use std::thread;

use error::Result;

const REDIS_KEY_EVENT_NOTIFICATION: &str = "__keyevent@0__:zadd";

pub struct RedisClient {
    client: redis::Client,
    conn: redis::Connection,
    tx: mpsc::Sender<String>,
    rx: mpsc::Receiver<String>,
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

    pub fn query_value(&self, key: String) -> Result<String> {
        let (key, score): (String, String) = self.get_with_block_height(key)?;
        self.get(format!("{}#{}", key, score))
    }

    #[rustfmt::skip]
    pub fn get_with_block_height(&self, key: String) -> Result<(String, String)> {
        let key_score: redis::Value = redis::cmd("ZREVRANGEBYSCORE")
            .arg(key)
            .arg("+inf").arg("-inf")
            .arg("WITHSCORES")
            .arg("LIMIT").arg(0).arg(1)
            .query(&self.conn)?;
        let (key, score): (String, String) = redis::from_redis_value(&key_score)?;
        debug!("key = {:?}, block_height = {:?}", key, score);
        Ok((key, score))
    }

    pub fn get(&self, key: String) -> Result<String> {
        let value = redis::cmd("GET").arg(key).query(&self.conn)?;
        let value = redis::from_redis_value(&value)?;
        Ok(value)
    }

    pub fn recv_key(&self) -> Result<String> {
        let key = self.rx.recv()?;
        Ok(key)
    }

    pub fn start_subscription(&self) -> Result<thread::JoinHandle<Result<()>>> {
        let tx = self.tx.clone();
        let mut sub_conn = self.client.get_connection()?;

        let thread = thread::spawn(move || {
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
        });

        Ok(thread)
    }
}
