pub fn get_value_prefix(value: &serde_json::Value) -> String {
    // unwrap() always not be panic, because the value is from block queue.
    serde_json::from_value(value["prefix"].clone()).unwrap()
}
