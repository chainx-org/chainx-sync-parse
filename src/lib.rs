#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate hex;
#[macro_use]
extern crate parity_codec_derive;
extern crate parity_codec;
extern crate parking_lot;
extern crate redis;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate strum_macros;
extern crate strum;
#[macro_use]
extern crate jsonrpc_macros;
extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
extern crate rayon;

// substrate core
extern crate sr_primitives;
extern crate sr_std;
extern crate substrate_primitives;
// substrate runtime module.
extern crate srml_metadata;
extern crate srml_support;

mod error;
mod parse;
mod serde_ext;
mod subscribe;
mod transmit;

use std::collections::BTreeMap;
use std::sync::Arc;

use parking_lot::RwLock;

pub type BlockQueue = Arc<RwLock<BTreeMap<u64, Vec<serde_json::Value>>>>;
pub use self::error::{Error, Result};
pub use self::parse::RuntimeStorage;
pub use self::serde_ext::Bytes;
pub use self::subscribe::RedisClient;
pub use self::transmit::RegisterService;
