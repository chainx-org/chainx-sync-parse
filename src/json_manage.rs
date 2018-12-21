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

pub fn read() -> Result<String> {
    let p = Path::new("./target/reg.json");
    let mut file = File::open(p)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

#[derive(Deserialize, Debug)]
struct JsonRpcResponse<T> {
    id: u64,
    jsonrpc: String,
    result: T,
}

pub fn post(url: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
    Ok(reqwest::Client::new()
        .post(url)
        .json(&body)
        .send()?
        .json::<serde_json::Value>()?)
}

pub fn deserialize<T: Debug + DeserializeOwned>(url: &str, body: &serde_json::Value) -> Result<T> {
    let resp: JsonRpcResponse<T> = serde_json::from_value(post(url, &body)?)?;
    Ok(resp.result)
}
