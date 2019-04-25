use std::collections::vec_deque::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::Result;

const BLOCK_SIZE: u64 = 1 << 16;
const BUFFER_SIZE: usize = 1 << 8;
const LINES_NUM: usize = 10;

struct Settings {
    mode: (u64, u8), // (number of lines, delimiter)
    sleep_milli: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            mode: (10, '\n' as u8),
            sleep_milli: 50,
        }
    }
}

pub struct Tail {
    tx: mpsc::Sender<Vec<u8>>,
    rx: mpsc::Receiver<Vec<u8>>,
}

impl Tail {
    pub fn new() -> Tail {
        let (tx, rx) = mpsc::channel();
        Tail { tx, rx }
    }

    //    pub fn run(&self, filename: &str) -> Result<JoinHandle<()>> {
    //        let path = Path::new(filename);
    //        assert!(!path.is_dir());
    //        let file = File::open(path)?;
    //        let mut reader = BufReader::new(file);
    //        let settings = Settings::default();
    //
    //        let tx = self.tx.clone();
    //        let handle = thread::spawn(move || loop {
    //            thread::sleep(Duration::from_millis(settings.sleep_milli));
    //            let mut lines: VecDeque<String> = VecDeque::with_capacity(LINES_NUM);
    //            let mut line = String::with_capacity(BUFFER_SIZE);
    //            loop {
    //                match reader.read_line(&mut line) {
    //                    Ok(0) => break,
    //                    Ok(_) => {
    //                        if lines.len() >= LINES_NUM {
    //                            let line = lines.pop_front().unwrap();
    //                            tx.send(line.into_bytes()).unwrap();
    //                        }
    //
    //                    },
    //                    Err(e) => error!("Tail read line error: {}", e),
    //                }
    //                line.clear();
    //            }
    //        });
    //        Ok(handle)
    //    }

    pub fn recv_key(&self) -> Result<Vec<u8>> {
        Ok(self.rx.recv()?)
    }
}

//fn filter_line() {
//
//}
