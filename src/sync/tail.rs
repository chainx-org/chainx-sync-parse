use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::Result;

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

    pub fn run(&self, file: File) -> Result<JoinHandle<()>> {
        let mut reader = BufReader::new(file);
        let tx = self.tx.clone();

        let handle = thread::spawn(move || {
            let mut line = Vec::with_capacity(BUFFER_SIZE);
            'outer: loop {
                thread::sleep(Duration::from_millis(50));
                loop {
                    line.clear();
                    match reader.read_until(b'\n', &mut line) {
                        Ok(0) => break 'outer,
                        Ok(_) => {
                            if let Some(data) = filter_line(&line) {
                                tx.send(data).unwrap()
                            } else {
                                continue;
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

fn filter_line(_line: &[u8]) -> Option<StorageData> {
    None
}

#[test]
fn test_tail() -> Result<()> {
    use std::path::Path;

    let path = Path::new("./tail.log");
    assert!(path.is_file());
    let file = File::open(path)?;

    let tail = Tail::new();
    let handle = tail.run(file)?;

    while let Ok(StorageData { h, key, value }) = tail.recv_data() {
        println!(
            "h = {:?}, key = {:?}, value = {:?}",
            h,
            String::from_utf8(key).unwrap(),
            String::from_utf8(value).unwrap()
        );
    }

    handle.join().expect("Join should be successful");
    Ok(())
}
