#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate hex;
extern crate parity_codec;
extern crate parking_lot;
extern crate redis;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
#[macro_use]
extern crate jsonrpc_macros;

// substrate runtime metadata module.
extern crate substrate_metadata;

pub mod error;
pub mod parse;
pub mod serde_ext;
pub mod subscribe;
pub mod register_server;

pub use std::collections::BTreeMap;
pub use std::sync::Arc;


pub use parking_lot::RwLock;

pub type BlockQueue = Arc<RwLock<BTreeMap<u64, serde_json::Value>>>;

pub use self::error::{Error, Result};
pub use self::parse::{get_runtime_metadata, parse_metadata};
pub use self::serde_ext::Bytes;
pub use self::subscribe::RedisClient;
pub use self::transmit::file_io;
pub use self::transmit::TransmitClient;
