use parity_codec::Decode;

use substrate_metadata::{
    DecodeDifferent, ModuleMetadata, RuntimeMetadata, RuntimeModuleMetadata, StorageMetadata,
};

use super::Result;

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
    let result = resp.result;
    let blob = hex::decode(&result[2..]).unwrap();
    let runtime_metadata: RuntimeMetadata = Decode::decode(&mut blob.as_slice()).unwrap();
    let runtime_modules_metadata = match runtime_metadata.modules {
        DecodeDifferent::Decoded(val) => val,
        _ => return Err("Decode runtime metadata failed".into()),
    };

    Ok(runtime_modules_metadata)
}

//pub fn parse_runtime_modules_metadata(modules_metadata: Vec<RuntimeModuleMetadata>) -> Result<()> {
//    Ok(())
//}
