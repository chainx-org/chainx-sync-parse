use jsonrpc_core::{Error as RpcError, Result as RpcResult};
use serde::de::Deserialize;

use crate::BlockQueue;

pub fn from_json_str<'a, T>(s: &'a str) -> RpcResult<T>
where
    T: Deserialize<'a>,
{
    match serde_json::from_str(s) {
        Ok(value) => Ok(value),
        Err(_) => Err(RpcError::parse_error()),
    }
}

pub fn get_value_prefix(value: &serde_json::Value) -> String {
    // unwrap() always not be panic, because the value is from block queue.
    serde_json::from_value(value["prefix"].clone()).unwrap()
}
