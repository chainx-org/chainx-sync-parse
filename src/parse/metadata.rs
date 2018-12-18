use parity_codec::Decode;
use substrate_metadata::{
    DecodeDifferent, RuntimeMetadata, RuntimeModuleMetadata, StorageFunctionMetadata,
    StorageFunctionModifier, StorageFunctionType,
};

use error::Result;

macro_rules! try_opt {
    ($expr:expr) => {{
        match $expr {
            DecodeDifferent::Decoded(val) => val,
            _ => return Err("Decode runtime metadata failed".into()),
        }
    }};
}

#[derive(Deserialize, Debug)]
struct RpcResponseContent<T> {
    id: u64,
    jsonrpc: String,
    result: T,
}

pub fn get_runtime_modules_metadata(url: &str) -> Result<Vec<RuntimeModuleMetadata>> {
    let req = json!({
        "jsonrpc": "2.0",
        "method": "state_getMetadata",
        "id": 1,
        "params": [],
    });

    let mut resp = reqwest::Client::new().post(url).json(&req).send()?;
    let resp = resp.json::<serde_json::Value>()?;
    let resp: RpcResponseContent<String> = serde_json::from_str(&resp.to_string())?;
    let blob = hex::decode(&resp.result[2..]).unwrap();
    let runtime_metadata: RuntimeMetadata = Decode::decode(&mut blob.as_slice()).unwrap();
    let module_metadata_array: Vec<RuntimeModuleMetadata> = try_opt!(runtime_metadata.modules);
    Ok(module_metadata_array)
}

// For help print storage metadata.
pub fn parse_metadata(module_metadata_array: Vec<RuntimeModuleMetadata>) -> Result<()> {
    for module_metadata in module_metadata_array {
        let storage_metadata = match module_metadata.storage {
            Some(DecodeDifferent::Decoded(val)) => val,
            _ => continue,
        };
        let prefix: String = try_opt!(storage_metadata.prefix);
        let func_metadata_array: Vec<StorageFunctionMetadata> =
            try_opt!(storage_metadata.functions);
        for func_metadata in func_metadata_array {
            let func_name: String = try_opt!(func_metadata.name);
            let (key, value) = match func_metadata.ty {
                StorageFunctionType::Plain(val) => {
                    let val: String = try_opt!(val);
                    match func_metadata.modifier {
                        StorageFunctionModifier::Optional => (
                            format!("{} {}", &prefix, &func_name),
                            format!("Option<{}>", val),
                        ),
                        StorageFunctionModifier::Default => {
                            (format!("{} {}", &prefix, &func_name), format!("{}", val))
                        }
                    }
                }
                StorageFunctionType::Map { key, value } => {
                    let key: String = try_opt!(key);
                    let value: String = try_opt!(value);
                    match func_metadata.modifier {
                        StorageFunctionModifier::Optional => (
                            format!("{} {} + {}", &prefix, &func_name, key),
                            format!("Option<{}>", value),
                        ),
                        StorageFunctionModifier::Default => (
                            format!("{} {} + {}", &prefix, &func_name, key),
                            format!("{}", value),
                        ),
                    }
                }
            };
            println!("{} => {}", key, value);
        }
        println!();
    }
    Ok(())
}

//pub fn create_type_mapping_table() {}
