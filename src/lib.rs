#[macro_use]
extern crate log;

pub mod cli;
mod error;
pub mod logger;
mod parse;
mod register;
#[cfg(feature = "sync-log")]
mod sync;
mod types;

use std::collections::BTreeMap;
use std::sync::Arc;

use parking_lot::RwLock;

pub use self::cli::CliConfig;
pub use self::error::{Error, Result};
pub use self::parse::RuntimeStorage;
pub use self::register::RegisterService;
#[cfg(feature = "sync-log")]
pub use self::sync::*;

pub type BlockQueue = Arc<RwLock<BTreeMap<u64, Vec<serde_json::Value>>>>;
