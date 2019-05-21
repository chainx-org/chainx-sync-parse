#[cfg(feature = "sync-redis")]
mod redis;
#[cfg(feature = "sync-redis")]
pub use self::redis::Redis;

#[cfg(feature = "sync-log")]
mod tail;
#[cfg(feature = "sync-log")]
pub use self::tail::Tail;
