use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use regex::bytes::Regex;

use crate::Result;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"(?-u)INFO msgbus\|height:\[(\d*)]\|key:\[(.*)]\|value:\[(.*)]").unwrap();
}

const BUFFER_SIZE: usize = 1 << 8;

type StorageData = (u64, Vec<u8>, Vec<u8>); // height, key, value

pub struct Tail {
    tx: mpsc::Sender<StorageData>,
    rx: mpsc::Receiver<StorageData>,
}

impl Default for Tail {
    fn default() -> Self {
        Self::new()
    }
}

impl Tail {
    pub fn new() -> Tail {
        let (tx, rx) = mpsc::channel();
        Tail { tx, rx }
    }

    pub fn run(&self, file: File) -> Result<thread::JoinHandle<()>> {
        let mut reader = BufReader::new(file);
        let tx = self.tx.clone();

        let handle = thread::spawn(move || {
            let mut line = Vec::with_capacity(BUFFER_SIZE);
            loop {
                thread::sleep(Duration::from_millis(50));
                loop {
                    line.clear();
                    match reader.read_until(b'\n', &mut line) {
                        Ok(0) => break,
                        Ok(_) => {
                            if let Some(data) = filter_line(&line) {
                                tx.send(data).unwrap();
                            }
                        }
                        Err(err) => error!("Tail read line error: {:?}", err),
                    }
                }
            }
        });
        Ok(handle)
    }

    pub fn recv_data(&self) -> Result<StorageData> {
        Ok(self.rx.recv()?)
    }
}

fn filter_line(line: &[u8]) -> Option<StorageData> {
    if let Some(caps) = RE.captures(line) {
        let height = std::str::from_utf8(&caps[1])
            .unwrap()
            .parse::<u64>()
            .expect("Parse height should not be failed");
        // key and value should be hex
        let key = hex::decode(&caps[2]).expect("Hex decode key should not be failed");
        let value = hex::decode(&caps[3]).expect("Hex decode value should not be failed");
        debug!(
            "msgbus|height:[{}]|key:[{}]|value:[{}]",
            height,
            hex::encode(&key),
            hex::encode(&value)
        );
        Some((height, key, value))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_capture_value() {
        // StorageValue - XBridgeOfBTC BestIndex
        let map = "INFO msgbus|height:[0]|key:[584272696467654f664254432042657374496e646578]|value:[267334e6d27bbdf2cbcd07c97675363c8c63f44c3e7ef0384a00000000000000]\n";
        let caps = RE.captures(map.as_bytes()).unwrap();
        println!(
            "height: {:?}\nkey: {:?}\nvalue = {:?}\n",
            &caps[1], &caps[2], &caps[3]
        );
        let height = std::str::from_utf8(&caps[1])
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let key = String::from_utf8(hex::decode(&caps[2]).unwrap()).unwrap();
        let value = hex::decode(&caps[3].to_vec()).unwrap();
        assert_eq!(height, 0);
        assert_eq!(key, "XBridgeOfBTC BestIndex".to_string());
        assert_eq!(
            value,
            hex::decode("267334e6d27bbdf2cbcd07c97675363c8c63f44c3e7ef0384a00000000000000")
                .unwrap(),
        );
    }

    #[test]
    fn test_regex_capture_map() {
        // StorageMap - XAssets AssetInfo\u{c}PCX
        let value = "INFO msgbus|height:[0]|key:[58417373657473204173736574496e666f0c504358]|value:[0c5043583c506f6c6b61646f7420436861696e58000800b0436861696e5827732063727970746f2063757272656e637920696e20506f6c6b61646f742065636f6c6f6779010000000000000000]\n";
        let caps = RE.captures(value.as_bytes()).unwrap();
        println!(
            "height: {:?}\nkey: {:?}\nvalue = {:?}",
            &caps[1], &caps[2], &caps[3]
        );
        let height = std::str::from_utf8(&caps[1])
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let key = String::from_utf8(hex::decode(&caps[2]).unwrap()).unwrap();
        let value = hex::decode(&caps[3].to_vec()).unwrap();
        assert_eq!(height, 0);
        assert_eq!(key, "XAssets AssetInfo\u{c}PCX".to_string());
        assert_eq!(
            value,
            hex::decode("0c5043583c506f6c6b61646f7420436861696e58000800b0436861696e5827732063727970746f2063757272656e637920696e20506f6c6b61646f742065636f6c6f6779010000000000000000").unwrap(),
        );
    }
}
