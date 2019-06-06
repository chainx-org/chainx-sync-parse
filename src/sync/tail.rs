use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader /*, Seek, SeekFrom*/};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use regex::bytes::Regex;

use crate::{CliConfig, Result};

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"(?-u)INFO msgbus\|height:\[(\d*)]\|key:\[(.*)]\|value:\[(.*)]").unwrap();
}

const BUFFER_SIZE: usize = 1024;

type StorageData = (u64, Vec<u8>, Vec<u8>); // (height, key, value)

pub struct Tail {
    tx: mpsc::Sender<StorageData>,
    rx: mpsc::Receiver<StorageData>,
}

impl Default for Tail {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx }
    }
}

impl Tail {
    pub fn new() -> Tail {
        Tail::default()
    }

    pub fn run(&self, config: &CliConfig) -> Result<thread::JoinHandle<()>> {
        let tx = self.tx.clone();
        let mut tail_impl = TailImpl::new(tx, config)?;
        let handle = thread::spawn(move || {
            tail_impl.run();
            error!(target: "parse", "Tail thread exists abnormally");
        });
        Ok(handle)
    }

    pub fn recv_data(&self) -> Result<StorageData> {
        Ok(self.rx.recv()?)
    }
}

pub struct TailImpl {
    tx: mpsc::Sender<StorageData>,
    start_height: u64,
    stop_height: u64,
    reader: BufReader<File>,
    line: Vec<u8>,
    /*
    /// A file lock that indicates whether the sync log file has a rotate event.
    lock_file: PathBuf,
    */
    /// A flag that indicates whether the genesis block has been scanned.
    is_genesis: bool,
}

impl TailImpl {
    pub fn new(tx: mpsc::Sender<StorageData>, config: &CliConfig) -> Result<Self> {
        info!(target: "parse", "Start reading sync log [path: {:?}]", &config.sync_log_path);
        let sync_log_file = read_sync_log_file(&config.sync_log_path)?;
        let reader = BufReader::with_capacity(10 * BUFFER_SIZE, sync_log_file);
        let line = Vec::with_capacity(BUFFER_SIZE);
        /*let lock_file = lock_file_path(&config.sync_log_path)?;*/
        Ok(Self {
            tx,
            start_height: config.start_height,
            stop_height: config.stop_height,
            reader,
            line,
            /*lock_file,*/
            is_genesis: true,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.line.clear();
            /*
            if self.should_rotate() {
                info!(target: "parse", "Start rotating sync log");
                if let Err(e) = self.rotate() {
                    error!(target: "parse", "Failed to rotate sync log: {:?}", e);
                }
                info!(target: "parse", "Finish rotating sync log");
            }
            */
            match self.reader.read_until(b'\n', &mut self.line) {
                Ok(0) => thread::sleep(Duration::from_millis(50)),
                Ok(_) => {
                    if let Some(data) = self.filter_line() {
                        let height = data.0;
                        if height > self.stop_height {
                            warn!(
                                target: "parse","Finish scanning, the process will EXIT in 10s...");
                            thread::sleep(Duration::from_secs(5));
                            std::process::exit(0);
                        }
                        if height >= self.start_height {
                            self.tx
                                .send(data)
                                .expect("Send sync data shouldn't be fail");
                        }
                    }
                }
                Err(err) => error!(target: "parse", "Failed to read the sync logs in buffer: {:?}", err),
            }
        }
    }

    /// Filter the sync log and extract the `msgbus` log data.
    fn filter_line(&mut self) -> Option<StorageData> {
        if let Some(caps) = RE.captures(&self.line) {
            let height = std::str::from_utf8(&caps[1])
                .unwrap()
                .parse::<u64>()
                .expect("Parse height should not be fail");

            // Ignore the block with height 0 (except genesis block)
            {
                if height != 0 {
                    self.is_genesis = false;
                }
                if !self.is_genesis && height == 0 {
                    return None;
                }
            }

            // Key and value should be hex
            let key = decode_hex("key", height, &caps[2]);
            let value = decode_hex("value", height, &caps[3]);
            record_sync_log(height, &key, &value);

            Some((height, key, value))
        } else {
            None
        }
    }

    /*
    fn filter_send(&mut self) {
        if let Some(data) = self.filter_line() {
            let height = data.0;
            if height >= self.start_height {
                self.tx
                    .send(data)
                    .expect("Send sync data shouldn't be fail");
            }
        }
    }

    /// Check whether LOCK file is exists.
    fn should_rotate(&mut self) -> bool {
        self.lock_file.exists()
    }

    /// Rotate the current file and delete LOCK file.
    fn rotate(&mut self) -> Result<()> {
        // Read all remaining logs in buffer
        loop {
            self.line.clear();
            match self.reader.read_until(b'\n', &mut self.line) {
                Ok(0) => {
                    info!(target: "parse", "Finish reading the remaining sync logs in buffer");
                    break;
                }
                Ok(_) => self.filter_send(),
                Err(err) => error!(
                    target: "parse",
                    "Failed to read the remaining sync logs in buffer: {:?}",
                    err
                ),
            }
        }
        let _ = self.reader.seek(SeekFrom::Start(0))?;
        info!(target: "parse", "Seek sync log to start position");
        self.delete_lock_file()?;
        Ok(())
    }

    /// Delete LOCK file
    fn delete_lock_file(&mut self) -> Result<()> {
        fs::remove_file(&self.lock_file)?;
        info!(target: "parse", "Deleted LOCK file");
        Ok(())
    }
    */
}

/// Opens sync log file. Creates a new log file if it doesn't exist.
fn read_sync_log_file(file_path: &Path) -> Result<File> {
    check_parent_dir(file_path)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;
    Ok(file)
}

fn check_parent_dir(file_path: &Path) -> Result<()> {
    let parent = file_path
        .parent()
        .ok_or("Unable to get parent directory of log file")?;
    if !parent.is_dir() {
        fs::create_dir_all(parent)?
    }
    Ok(())
}

/*
fn lock_file_path(file_path: &Path) -> Result<PathBuf> {
    let parent = file_path
        .parent()
        .ok_or("Unable to get parent directory of log file")?;
    let mut parent = parent.to_path_buf();
    parent.push("LOCK");
    Ok(parent)
}
*/

fn decode_hex(name: &str, height: u64, cap: &[u8]) -> Vec<u8> {
    hex::decode(cap).unwrap_or_else(|_| {
        panic!(
            "Decoding hex {} fail: block #{}, key={:?}",
            name, height, cap
        )
    })
}

fn record_sync_log(height: u64, key: &[u8], value: &[u8]) {
    info!(
        target: "msgbus",
        "msgbus|height:[{}]|key:[{}]|value:[{}]",
        height,
        hex::encode(key),
        hex::encode(value)
    );
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
