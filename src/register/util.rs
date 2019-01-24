use crate::BlockQueue;

pub fn from_value(value: serde_json::Value) -> String {
    serde_json::from_value(value).unwrap()
}

pub fn get_value_prefix(value: &serde_json::Value) -> String {
    // unwrap() won't be panic, because the value is from block queue.
    serde_json::from_value(value["prefix"].clone()).unwrap()
}

/// Get the max key of BTreeMap, which is max block height of block queue.
pub fn get_max_block_height(queue: &BlockQueue) -> u64 {
    *queue.read().keys().next_back().unwrap()
}

/// Get the min key of BTreeMap, which is min block height of block queue.
pub fn get_min_block_height(queue: &BlockQueue) -> u64 {
    *queue.read().keys().next().unwrap()
}
