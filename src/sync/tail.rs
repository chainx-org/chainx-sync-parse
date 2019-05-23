//mod logger;

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration as StdDuration;

use regex::bytes::Regex;

use crate::Result;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"(?-u)INFO msgbus\|height:\[(\d*)]\|key:\[(.*)]\|value:\[(.*)]").unwrap();
}

const BUFFER_SIZE: usize = 1024;
const BLOCK_NUMBER_PER_LOG_FILE: u64 = 50000;

type StorageData = (u64, Vec<u8>, Vec<u8>); // (height, key, value)

pub struct Tail {
    tx: mpsc::Sender<StorageData>,
    rx: mpsc::Receiver<StorageData>,
}

impl Tail {
    pub fn new() -> Tail {
        let (tx, rx) = mpsc::channel();
        Tail { tx, rx }
    }

    pub fn run(&self, file_path: impl AsRef<Path>) -> Result<thread::JoinHandle<()>> {
        let file_path = file_path.as_ref().to_path_buf();
        let file = open_log_file(&file_path)?;
        let file_copy = file.try_clone()?;
        let tx = self.tx.clone();

        let handle = thread::spawn(move || sync_log_data(file, file_copy, &tx));
        Ok(handle)
    }

    pub fn recv_data(&self) -> Result<StorageData> {
        Ok(self.rx.recv()?)
    }
}

/// Opens sync log file. Creates a new log file if it doesn't exist.
fn open_log_file(path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref();
    let parent = path
        .parent()
        .expect("Unable to get parent directory of log file");
    if !parent.is_dir() {
        fs::create_dir_all(parent)?
    }

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
}

fn sync_log_data(file: File, file_copy: File, tx: &mpsc::Sender<StorageData>) {
    let mut reader = BufReader::new(file);
    let mut line = Vec::with_capacity(BUFFER_SIZE);
    loop {
        thread::sleep(StdDuration::from_millis(50));
        loop {
            line.clear();
            match reader.read_until(b'\n', &mut line) {
                Ok(0) => break,
                Ok(_) => {
                    if let Some(data) = filter_line(&line) {
                        let height = data.0;
                        if height % BLOCK_NUMBER_PER_LOG_FILE == 0 {
                            file_copy
                                .set_len(0)
                                .expect("Setting the length of underlying file shouldn't be fail");
                            reader
                                .seek(SeekFrom::Start(0))
                                .expect("Seek the cursor of file shouldn't be fail");
                            info!("Split sync node log, current block height #{}", height);
                        }
                        tx.send(data).expect("Send sync data shouldn't be fail");
                    }
                }
                Err(err) => error!("Tail read line error: {:?}", err),
            }
        }
    }
}

/// Filter the sync log and extract the `msgbus` log data.
fn filter_line(line: &[u8]) -> Option<StorageData> {
    if let Some(caps) = RE.captures(line) {
        let height = std::str::from_utf8(&caps[1])
            .unwrap()
            .parse::<u64>()
            .expect("Parse height should not be fail");
        // key and value should be hex
        let key = hex::decode(&caps[2]).expect("Hex decode key should not be fail");
        let value = hex::decode(&caps[3]).expect("Hex decode value should not be fail");
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
