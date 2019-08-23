#[cfg(feature = "sync-log")]
mod tail;
#[cfg(feature = "sync-log")]
pub use self::tail::Tail;
