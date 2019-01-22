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

pub fn get_max_height(queue: &BlockQueue) -> u64 {
    match queue.read().keys().next_back() {
        Some(key) => *key,
        None => 0,
    }
}

pub fn get_min_height(queue: &BlockQueue) -> u64 {
    *queue.read().keys().next().unwrap()
}