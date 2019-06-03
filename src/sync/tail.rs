use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::Duration as StdDuration;

use chrono::{Date, Local};
use regex::bytes::Regex;

use crate::Result;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"(?-u)INFO msgbus\|height:\[(\d*)]\|key:\[(.*)]\|value:\[(.*)]").unwrap();
}

const BUFFER_SIZE: usize = 1024;
const ROTATE_DELAY_SECONDS: u64 = 10; // 10 seconds

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

    pub fn run(
        &self,
        file_path: impl AsRef<Path>,
        start_height: u64,
    ) -> Result<thread::JoinHandle<()>> {
        let tx = self.tx.clone();
        let mut filter_and_rotate = FilterAndRotate::new(file_path, start_height, tx)?;

        let handle = thread::spawn(move || filter_and_rotate.run());
        Ok(handle)
    }

    pub fn recv_data(&self) -> Result<StorageData> {
        Ok(self.rx.recv()?)
    }
}

/// This FileLogger rotates logs according to block height.
/// After rotating, the original log file would be renamed to "{original name}.{height}"
/// Note: log file will *not* be compressed or otherwise modified.
pub struct FilterAndRotate {
    tx: mpsc::Sender<StorageData>,
    file_path: PathBuf,
    fd: RawFd,
    reader: BufReader<File>,
    next_rotation_time: Date<Local>,
    start_height: u64,
    /// Indicates whether the genesis block has been scanned
    is_genesis: bool,
}

impl FilterAndRotate {
    pub fn new(
        file_path: impl AsRef<Path>,
        start_height: u64,
        tx: mpsc::Sender<StorageData>,
    ) -> io::Result<Self> {
        let sync_log_path = file_path.as_ref().to_path_buf();
        let sync_log_file = read_sync_log_file(&sync_log_path)?;
        let fd = sync_log_file.as_raw_fd();
        info!("Sync log file path: {:?}, file description: {:?}", sync_log_path, fd);
        let reader = BufReader::with_capacity(10 * BUFFER_SIZE, sync_log_file);
        let next_rotation_time = compute_next_rotation_time();

        Ok(Self {
            tx,
            file_path: file_path.as_ref().to_path_buf(),
            fd,
            reader,
            next_rotation_time,
            start_height,
            is_genesis: true,
        })
    }

    pub fn run(&mut self) {
        let mut line = Vec::with_capacity(BUFFER_SIZE);
        loop {
            line.clear();
            if self.should_rotate() {
                // Waiting for the logrotate task to complete
                thread::sleep(StdDuration::from_secs(ROTATE_DELAY_SECONDS));
                info!("Start rotating sync log, datetime: {:?}", Local::now());
                let _ = self.rotate();
                info!("Finish rotating sync log");
            }
            match self.reader.read_until(b'\n', &mut line) {
                Ok(0) => thread::sleep(StdDuration::from_millis(50)),
                Ok(_) => {
                    if let Some(data) = filter_line(&line, &mut self.is_genesis) {
                        let height = data.0;
                        if height >= self.start_height {
                            self.tx
                                .send(data)
                                .expect("Send sync data shouldn't be fail");
                        }
                    }
                }
                Err(err) => error!("Tail read line error: {:?}", err),
            }
        }
    }

    fn should_rotate(&mut self) -> bool {
        Local::today() == self.next_rotation_time
    }

    /// Rotates the current file and updates the next rotation time.
    fn rotate(&mut self) -> io::Result<()> {
        let sync_log_file = File::open(&self.file_path)?;
        let new_fd = sync_log_file.as_raw_fd();
        info!("Sync log file description: {:?}, new file description: {:?}", self.fd, new_fd);
        self.fd = new_fd;
        self.reader = BufReader::new(sync_log_file);
        self.update_next_rotation_time();
        Ok(())
    }

    /// Updates the next rotation time.
    fn update_next_rotation_time(&mut self) {
        self.next_rotation_time = compute_next_rotation_time();
    }
}

/// Compute next rotate time.
fn compute_next_rotation_time() -> Date<Local> {
    let now = Local::today();
    let next = now.succ();
    next
}

/// Opens sync log file. Creates a new log file if it doesn't exist.
fn read_sync_log_file(file_path: &Path) -> io::Result<File> {
    check_parent_dir(file_path)?;
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
}

fn check_parent_dir(file_path: &Path) -> io::Result<()> {
    let parent = file_path
        .parent()
        .expect("Unable to get parent directory of log file");
    if !parent.is_dir() {
        fs::create_dir_all(parent)?
    }
    Ok(())
}

/// Filter the sync log and extract the `msgbus` log data.
fn filter_line(line: &[u8], is_genesis: &mut bool) -> Option<StorageData> {
    if let Some(caps) = RE.captures(line) {
        let height = std::str::from_utf8(&caps[1])
            .unwrap()
            .parse::<u64>()
            .expect("Parse height should not be fail");

        // Ignore the block with height 0 (except genesis block)
        {
            if height != 0 {
                *is_genesis = false;
            }
            if !*is_genesis && height == 0 {
                return None;
            }
        }

        // Key and value should be hex
        let key = hex::decode(&caps[2]).expect(&format!(
            "Hex decode key should not be fail: block #{}, key={:?}",
            height, &caps[2]
        ));
        let value = hex::decode(&caps[3]).expect(&format!(
            "Hex decode value should not be fail: block #{}, value={:?}",
            height, &caps[3]
        ));
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
