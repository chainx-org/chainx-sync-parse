#[macro_use(slog_o, slog_error, slog_warn, slog_info, slog_debug)]
extern crate slog;
#[macro_use]
extern crate slog_scope;
#[cfg(feature = "pgsql")]
#[macro_use]
extern crate diesel;

mod cli;
mod error;
pub mod logger;
mod parse;
#[cfg(feature = "pgsql")]
mod pgsql;
mod register;
mod sync;
mod types;

use std::collections::BTreeMap;
use std::sync::Arc;

use parking_lot::RwLock;

pub use self::cli::CliConfig;
pub use self::error::{Error, Result};
pub use self::parse::RuntimeStorage;
#[cfg(feature = "pgsql")]
pub use self::pgsql::*;
pub use self::register::RegisterService;
pub use self::sync::{Redis, Tail};

pub type BlockQueue = Arc<RwLock<BTreeMap<u64, Vec<serde_json::Value>>>>;
