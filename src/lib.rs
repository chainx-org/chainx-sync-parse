mod error;
mod parse;
mod register;
mod serde_ext;
mod subscribe;

use std::collections::BTreeMap;
use std::sync::Arc;

use parking_lot::RwLock;

pub use self::error::{Error, Result};
pub use self::parse::RuntimeStorage;
pub use self::register::RegisterService;
pub use self::serde_ext::Bytes;
pub use self::subscribe::RedisClient;

pub type BlockQueue = Arc<RwLock<BTreeMap<u64, Vec<serde_json::Value>>>>;
