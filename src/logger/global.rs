//! Copy from https://github.com/breeswish/slog-global/blob/master/lib.rs
//!
//! Global loggers for [slog-rs].
//!
//! Provides a set of logging macros to free user from manually passing [`Logger`] objects around.
//!
//! This crate is similar to [slog-scope], but is simpler and faster. Also replacing macros will
//! be less likely to break existing code base.
//!
//! Not advised to be used in libraries.

use std::sync::Arc;

use arc_swap::{ArcSwap, Lease};
use slog::Logger;

/// Creates a logger that simply discards everything.
fn discard_logger() -> Logger {
    Logger::root(slog::Discard, slog_o!())
}

lazy_static::lazy_static! {
    static ref GLOBAL_LOGGER: ArcSwap<Logger> = ArcSwap::from(Arc::new(discard_logger()));
}

/// Sets the global `Logger`.
pub fn set_global(l: slog::Logger) {
    GLOBAL_LOGGER.store(Arc::new(l));
}

/// Gets the global `Logger`.
///
/// If you only want to access the global logger temporarily (i.e. as a local variable on stack but
/// not structures), use `borrow_global()` which is more efficient.
pub fn get_global() -> Arc<Logger> {
    GLOBAL_LOGGER.load()
}

/// Temporary borrows the global `Logger`.
pub fn borrow_global() -> Lease<Arc<Logger>> {
    GLOBAL_LOGGER.lease()
}

/// Clears the global `Logger` and discard future logging.
pub fn clear_global() {
    GLOBAL_LOGGER.store(Arc::new(discard_logger()));
}

/// Logs a critical level message using the global logger.
#[macro_export]
macro_rules! crit( ($($args:tt)+) => {
    ::slog::slog_crit![$crate::logger::global::borrow_global(), $($args)+]
};);

/// Logs a error level message using the global logger.
#[macro_export]
macro_rules! error( ($($args:tt)+) => {
    ::slog::slog_error![$crate::logger::global::borrow_global(), $($args)+]
};);

/// Logs a warning level message using the global logger.
#[macro_export]
macro_rules! warn( ($($args:tt)+) => {
    ::slog::slog_warn![$crate::logger::global::borrow_global(), $($args)+]
};);

/// Logs a info level message using the global logger.
#[macro_export]
macro_rules! info( ($($args:tt)+) => {
    ::slog::slog_info![$crate::logger::global::borrow_global(), $($args)+]
};);

/// Logs a debug level message using the global logger.
#[macro_export]
macro_rules! debug( ($($args:tt)+) => {
    ::slog::slog_debug![$crate::logger::global::borrow_global(), $($args)+]
};);

/// Logs a trace level message using the global logger.
#[macro_export]
macro_rules! trace( ($($args:tt)+) => {
    ::slog::slog_trace![$crate::logger::global::borrow_global(), $($args)+]
};);
