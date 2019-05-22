//#[macro_use(slog_o)]
//extern crate slog;
#[macro_use]
extern crate log;
#[cfg(feature = "pgsql")]
#[macro_use]
extern crate diesel;

mod error;
//#[macro_use]
//pub mod logger;
mod parse;
#[cfg(feature = "pgsql")]
mod pgsql;
mod register;
#[cfg(any(feature = "sync-redis", feature = "sync-log"))]
mod sync;
mod types;

use std::collections::BTreeMap;
use std::sync::Arc;

use parking_lot::RwLock;

pub use self::error::{Error, Result};
pub use self::parse::RuntimeStorage;
#[cfg(feature = "pgsql")]
pub use self::pgsql::*;
pub use self::register::RegisterService;
#[cfg(any(feature = "sync-redis", feature = "sync-log"))]
pub use self::sync::*;

pub type BlockQueue = Arc<RwLock<BTreeMap<u64, Vec<serde_json::Value>>>>;
