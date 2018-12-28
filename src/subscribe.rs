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

    pub fn query(&self, key: &str) -> Result<(u64, Vec<u8>)> {
        let (key, score): (String, u64) = self.query_key_and_score(key)?;
        let value = self.query_value(format!("{}", key))?;
        Ok((score, value))
    }

    #[rustfmt::skip]
    pub fn query_key_and_score(&self, key: &str) -> Result<(String, u64)> {
        let (key, score): (String, u64) = redis::cmd("ZREVRANGEBYSCORE")
            .arg(key)
            .arg("+inf").arg("-inf")
            .arg("WITHSCORES")
            .arg("LIMIT").arg(0).arg(1)
            .query(&self.conn)?;
//        println!("key: {:?}, score: {:?}", key, score);
        Ok((key, score))
    }

    pub fn query_value(&self, key: String) -> Result<Vec<u8>> {
        let value: Vec<u8> = redis::cmd("GET").arg(key).query(&self.conn)?;
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
