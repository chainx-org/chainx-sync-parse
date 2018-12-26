#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate hex;
extern crate parity_codec;
#[macro_use]
extern crate parity_codec_derive;
extern crate parking_lot;
extern crate redis;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate strum;
#[macro_use]
extern crate strum_macros;

// substrate core
extern crate sr_primitives;
extern crate sr_std;
extern crate substrate_primitives;
// substrate runtime metadata module.
extern crate srml_metadata;

pub mod error;
pub mod parse;
pub mod serde_ext;
pub mod subscribe;

pub use std::collections::BTreeMap;
pub use std::sync::Arc;

pub use parking_lot::RwLock;

pub type BlockQueue = Arc<RwLock<BTreeMap<u64, serde_json::Value>>>;

pub use self::error::{Error, Result};
pub use self::parse::test_parse_match;
pub use self::serde_ext::Bytes;
pub use self::subscribe::RedisClient;
