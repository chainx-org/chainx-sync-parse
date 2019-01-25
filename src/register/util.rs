use crate::BlockQueue;

pub fn get_value_prefix(value: &serde_json::Value) -> String {
    // unwrap() won't be panic, because the value is from block queue.
    serde_json::from_value(value["prefix"].clone()).unwrap()
}

/// Get the max key of BTreeMap, which is max block height of block queue.
#[inline]
pub fn get_max_block_height(queue: &BlockQueue) -> u64 {
    *queue.read().keys().next_back().unwrap()
}

/// Get the min key of BTreeMap, which is min block height of block queue.
#[inline]
pub fn get_min_block_height(queue: &BlockQueue) -> u64 {
    *queue.read().keys().next().unwrap()
}

#[inline]
pub fn get_block_queue_len(queue: &BlockQueue) -> usize {
    queue.read().len()
}
