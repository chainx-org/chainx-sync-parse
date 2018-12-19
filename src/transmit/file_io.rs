use error::Result;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn write(json: String) -> Result<()> {
    let p = Path::new("./target/reg.json");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&p)?;

    file.write(json.as_bytes())?;
    Ok(())
}

pub fn read_string() -> Result<String> {
    let p = Path::new("./target/reg.json");
    let mut file = File::open(p)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

pub fn deserialize<T: Debug + DeserializeOwned>(string: String) -> Result<T> {
    let resp: T = serde_json::from_str(string.as_str())?;
    Ok(resp)
}
