#[macro_export]
macro_rules! to_json {
    ($prefix:ident, $value:ident => $v:ident) => {
        to_json_impl!("value", $prefix, null, $value => $v)
    };

    ($prefix:ident, $key:ident => $k:ident, $value:ident => $v:ident) => {
        {
            *$k = match Decode::decode(&mut $key) {
                Some(key) => key,
                None => {
                    let err = format!("Decode failed, prefix: {:?}, key: {:?}", $prefix, $k);
                    error!("Runtime storage parse error: {:?}", err);
                    return Err(err.into());
                }
            };
            to_json_impl!("map", $prefix, $k, $value => $v)
        }
    };
}

macro_rules! to_json_impl {
    ($type:expr, $prefix:ident, $k:ident, $value:ident => $v:ident) => {{
        if $value.is_empty() {
            debug!("Empty Value: [{:?}] may have been removed", $prefix);
            return Ok(build_json!($type, $prefix, $k, null));
        }
        *$v = match Decode::decode(&mut $value.as_slice()) {
            Some(value) => value,
            None => {
                let err = format!("Decode failed, prefix: {:?}, value: {:?}", $prefix, $v);
                error!("Runtime storage parse error: {:?}", err);
                return Err(err.into());
            }
        };
        Ok(build_json!($type, $prefix, $k, $v))
    }};
}

macro_rules! build_json {
    ($type:expr, $prefix:ident, $key:ident, $value:ident) => {
        serde_json::json!({"type":$type, "prefix":$prefix, "key":$key, "value":$value})
    };
}
