use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::Duration as StdDuration;

use regex::bytes::Regex;

use crate::Result;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"(?-u)INFO msgbus\|height:\[(\d*)]\|key:\[(.*)]\|value:\[(.*)]").unwrap();
}

const BUFFER_SIZE: usize = 1024;
const BLOCK_NUMBER_PER_LOG_FILE: u64 = 3;

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
    height: u64,
    last_height: u64,
    reader: BufReader<File>,
    writer: BufWriter<File>,
    /// Sync log file path
    file_path: PathBuf,
    /// Indicates whether the genesis block has been scanned
    is_genesis: bool,
}

impl Drop for FilterAndRotate {
    fn drop(&mut self) {
        let _ = self.writer.flush();
    }
}

impl FilterAndRotate {
    pub fn new(
        file_path: impl AsRef<Path>,
        start_height: u64,
        tx: mpsc::Sender<StorageData>,
    ) -> io::Result<Self> {
        let from_file_path = file_path.as_ref().to_path_buf();
        let from_file = open_log_file(&from_file_path)?;
        let reader = BufReader::new(from_file);

        let to_file_path = rotation_file_path_with_height(file_path.as_ref(), start_height);
        let to_file = open_log_file(&to_file_path)?;
        let writer = BufWriter::new(to_file);

        Ok(Self {
            tx,
            height: start_height,
            last_height: start_height,
            reader,
            writer,
            file_path: file_path.as_ref().to_path_buf(),
            is_genesis: true,
        })
    }

    pub fn run(&mut self) {
        let mut line = Vec::with_capacity(BUFFER_SIZE);
        loop {
            line.clear();
            match self.reader.read_until(b'\n', &mut line) {
                Ok(0) => thread::sleep(StdDuration::from_millis(50)),
                Ok(_) => {
                    if let Some(data) = filter_line(&line, &mut self.is_genesis) {
                        self.height = data.0;
                        if self.should_rotate() {
                            self.rotate().expect("Rotate log shouldn't be fail");
                            info!("Split sync node log, current block height #{}", self.height);
                        }
                        self.last_height = self.height;

                        self.tx
                            .send(data)
                            .expect("Send sync data shouldn't be fail");
                    }
                    let _ = self.writer.write(&line);
                }
                Err(err) => error!("Tail read line error: {:?}", err),
            }
        }
    }

    fn should_rotate(&mut self) -> bool {
        (self.height != 0)
            && (self.height != self.last_height)
            && (self.height % BLOCK_NUMBER_PER_LOG_FILE == 0)
    }

    /// Rotates the current file and updates the next rotation time.
    fn rotate(&mut self) -> io::Result<()> {
        self.flush()?;

        // Note: renaming files while they're open only works on Linux and macOS.
        let new_to_path = rotation_file_path_with_height(&self.file_path, self.height);
        let new_to_file = open_log_file(&new_to_path)?;
        self.writer = BufWriter::new(new_to_file);
        Ok(())
    }

    /// Flushes the log file, without rotation.
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

/// Rotates file path with given block height.
fn rotation_file_path_with_height(file_path: &Path, height: u64) -> PathBuf {
    let mut file_path = file_path.as_os_str().to_os_string();
    file_path.push(format!(".{}", height));
    file_path.into()
}

/// Opens sync log file. Creates a new log file if it doesn't exist.
fn open_log_file(file_path: &Path) -> io::Result<File> {
    let parent = file_path
        .parent()
        .expect("Unable to get parent directory of log file");
    if !parent.is_dir() {
        fs::create_dir_all(parent)?
    }

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
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

//fn sync_log_data(file: File, file_copy: File, tx: &mpsc::Sender<StorageData>) {
//    let mut reader = BufReader::new(file);
//    let mut line = Vec::with_capacity(BUFFER_SIZE);
//
//    loop {
//        line.clear();
//        match reader.read_until(b'\n', &mut line) {
//            Ok(0) => thread::sleep(StdDuration::from_millis(50)),
//            Ok(_) => {
//                info!("{}", std::str::from_utf8(&line).unwrap_or(""));
//                if let Some(data) = filter_line(&line) {
//                    let height = data.0;
//                    if height % BLOCK_NUMBER_PER_LOG_FILE == 0 {
//                        file_copy
//                            .set_len(0)
//                            .expect("Setting the length of underlying file shouldn't be fail");
//                        reader
//                            .seek(SeekFrom::Start(0))
//                            .expect("Seek the cursor of file shouldn't be fail");
//                        info!("Split sync node log, current block height #{}", height);
//                    }
//                    tx.send(data).expect("Send sync data shouldn't be fail");
//                }
//            }
//            Err(err) => error!("Tail read line error: {:?}", err),
//        }
//    }
//}

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
