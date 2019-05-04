use std::collections::hash_map::Entry;
use std::sync::Arc;

use jsonrpc_derive::rpc;
use parking_lot::Mutex;
use semver::Version;

use super::{Context, RegisterService};
use crate::Result;

/// Register API
#[rpc]
pub trait RegisterApi {
    /// Register
    #[rpc(name = "register")]
    fn register(&self, prefixes: Vec<String>, url: String, version: String) -> Result<String>;

    /// Deregister
    #[rpc(name = "deregister")]
    fn deregister(&self, url: String) -> Result<String>;
}

impl RegisterApi for RegisterService {
    fn register(&self, prefixes: Vec<String>, url: String, version: String) -> Result<String> {
        let register_info = format!(
            "url: {:?}, prefix: {:?}, version: {:?}",
            &url, &prefixes, &version
        );
        let version = Version::parse(&version)?;
        match self.map.write().entry(url.clone()) {
            Entry::Occupied(mut entry) => {
                info!("Existing Register [{}]", register_info);
                let ctxt = entry.get_mut();
                ctxt.lock().update_prefixes(prefixes, version);
            }
            Entry::Vacant(entry) => {
                info!("New Register [{}]", register_info);
                let tx = self.tx.lock().clone();
                let ctxt = Arc::new(Mutex::new(Context::new(prefixes, version)));
                self.spawn_new_push(url, ctxt.clone(), tx);
                entry.insert(ctxt);
            }
        }
        Ok("OK".to_string())
    }

    fn deregister(&self, url: String) -> Result<String> {
        match self.map.write().entry(url.clone()) {
            Entry::Occupied(mut entry) => {
                info!("Deregister [{}]", url);
                let ctxt = entry.get_mut();
                ctxt.lock().deregister = true;
                Ok("OK".to_string())
            }
            Entry::Vacant(_) => Err("Nonexistent register url".into()),
        }
    }
}
