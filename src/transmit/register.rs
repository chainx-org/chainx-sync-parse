use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};

use crate::Result;

const REGISTER_RECORD_PATH: &str = "register.json";

pub type RegisterInfo = Arc<Mutex<Info>>;
pub type RegisterList = Arc<RwLock<HashMap<String, RegisterInfo>>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Info {
    pub prefix: Vec<String>,
    pub status: Status,
    pub version: String,
}

impl Info {
    pub fn new(prefix: Vec<String>, version: String) -> Self {
        Self {
            prefix,
            status: Status::default(),
            version,
        }
    }

    pub fn new_version(&mut self, version: String) {
        self.prefix.clear();
        self.status = Status::default();
        self.version = version;
        self.switch_on();
    }

    pub fn add_prefix(&mut self, prefix: String) {
        self.prefix.push(prefix);
        self.switch_on();
    }

    pub fn add_height(&mut self) {
        self.status.height += 1;
    }

    pub fn switch_on(&mut self) {
        if self.status.down {
            self.status.down = false;
        }
    }

    pub fn switch_off(&mut self) {
        if !self.status.down {
            self.status.down = true;
        }
    }
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct Status {
    pub height: u64,
    pub down: bool,
}

pub struct RegisterRecord;

impl RegisterRecord {
    pub fn save(json: &str) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&Path::new(REGISTER_RECORD_PATH))?;

        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load() -> Result<Option<String>> {
        match File::open(Path::new(REGISTER_RECORD_PATH)) {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;
                Ok(Some(buf))
            }
            Err(_) => Ok(None),
        }
    }
}
