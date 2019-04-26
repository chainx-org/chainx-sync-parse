use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use regex::bytes::Regex;

use crate::Result;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"(?-u)msgbus\|height:\[(\d*)]\|key:\[(.*)]\|value:\[(.*)]").unwrap();
}

const BUFFER_SIZE: usize = 1 << 8;

pub struct StorageData {
    height: u64,
    key: Vec<u8>,
    value: Vec<u8>,
}

pub struct Tail {
    tx: mpsc::Sender<StorageData>,
    rx: mpsc::Receiver<StorageData>,
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
                        Err(e) => error!("Tail read line error: {}", e),
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
            .unwrap();
        // key and value will be hex
//        let key = hex::decode(&caps[2]).unwrap();
//        let value = hex::decode(&caps[3]).unwrap();
        let key = (&caps[2]).to_vec();
        let value = (&caps[3]).to_vec();
        Some(StorageData { height, key, value })
    } else {
        None
    }
}

#[test]
fn test_tail() -> Result<()> {
    let path = std::path::Path::new("./tail.log");
    assert!(path.is_file());
    let file = File::open(path)?;

    let tail = Tail::new();
    let handle = tail.run(file)?;

    while let Ok(StorageData { height, key, value }) = tail.recv_data() {
        println!(
            "height = {:?}, key = {:?}, value = {:?}",
            height,
            String::from_utf8(key).unwrap(),
            String::from_utf8(value).unwrap(),
        );
    }

    handle.join().expect("Join should be successful");
    Ok(())
}

#[test]
fn test_regex_capture() {
    let re = Regex::new(r"(?-u)msgbus\|height:\[(\d*)]\|key:\[(.*)]\|value:\[(.*)]").unwrap();

    // StorageMap
    let map = b"msgbus|height:[123]|key:[XAssets AssetBalance\xa20\x81\x87C\x9a\xc2\x04\xdf\x9e)\x9e\x1eT\xaf\xef\xaf\xeaK\xf3H\xe0=\xadg\x977\xc9\x18q\xdcS\x0cPCX#123]|value:[\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x80|\x81J\x00\x00\x00\x00]\n";
    let caps = re.captures(map).unwrap();
    println!(
        "height: {:?}\nkey: {:?}\nvalue = {:?}",
        &caps[1], &caps[2], &caps[3]
    );
    let height = std::str::from_utf8(&caps[1])
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let key = (&caps[2]).to_vec();
    let value = (&caps[3]).to_vec();
    assert_eq!(height, 123);
    assert_eq!(
        key,
        vec![
            88, 65, 115, 115, 101, 116, 115, 32, 65, 115, 115, 101, 116, 66, 97, 108, 97, 110, 99,
            101, 162, 48, 129, 135, 67, 154, 194, 4, 223, 158, 41, 158, 30, 84, 175, 239, 175, 234,
            75, 243, 72, 224, 61, 173, 103, 151, 55, 201, 24, 113, 220, 83, 12, 80, 67, 88, 35, 49,
            50, 51
        ]
    );
    assert_eq!(
        value,
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 128, 124, 129, 74, 0, 0, 0, 0]
    );

    // StorageValue
    let value = b"msgbus|height:[123]|key:[XBridgeOfBTC BestIndex#123]|value:[T\xdd\x8c\xc9\xea\xc2\x80\xcc\x98&\\\x07\x9dvAH7\x89+;\xcf\x88\x9b\\k\x0e\x00\x00\x00\x00\x00\x00]\n";
    let caps = re.captures(value).unwrap();
    println!(
        "height: {:?}\nkey: {:?}\nvalue = {:?}",
        &caps[1], &caps[2], &caps[3]
    );
    let height = std::str::from_utf8(&caps[1])
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let key = (&caps[2]).to_vec();
    let value = (&caps[3]).to_vec();
    assert_eq!(height, 123);
    assert_eq!(
        key,
        vec![
            88, 66, 114, 105, 100, 103, 101, 79, 102, 66, 84, 67, 32, 66, 101, 115, 116, 73, 110,
            100, 101, 120, 35, 49, 50, 51
        ]
    );
    assert_eq!(
        value,
        vec![
            84, 221, 140, 201, 234, 194, 128, 204, 152, 38, 92, 7, 157, 118, 65, 72, 55, 137, 43,
            59, 207, 136, 155, 92, 107, 14, 0, 0, 0, 0, 0, 0
        ]
    );
}
