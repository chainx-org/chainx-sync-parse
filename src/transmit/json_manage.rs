use std::fmt::Debug;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use serde::de::DeserializeOwned;

use error::Result;

const FILE_PATH: &str = "./target/reg.json";

pub struct IO;

impl IO {
    pub fn write(json: String) -> Result<()> {
        let p = Path::new(FILE_PATH);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&p)?;

        file.write(json.as_bytes())?;
        Ok(())
    }

    pub fn read() -> Result<Option<String>> {
        let p = Path::new(FILE_PATH);
        match File::open(p) {
            Ok(mut file) => {
                let mut string = String::new();
                file.read_to_string(&mut string)?;
                Ok(Some(string))
            }
            Err(_) => Ok(None),
        }
    }
}

#[derive(Deserialize, Debug)]
struct JsonResponse<T> {
    result: T,
}

pub fn post(url: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
    Ok(reqwest::Client::new()
        .post(url)
        .json(&body)
        .send()?
        .json::<serde_json::Value>()?)
}

pub fn request<T: Debug + DeserializeOwned>(url: &str, body: &serde_json::Value) -> Result<T> {
    let resp: JsonResponse<T> = serde_json::from_value(post(url, &body)?)?;
    Ok(resp.result)
}
