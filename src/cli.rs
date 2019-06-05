use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "chainx-sync-parse",
    author = "ChainX <https://chainx.org>",
    about = "Synchronize and parse ChainX sync data"
)]
pub struct CliConfig {
    /// Specify the port of register service
    #[structopt(
        short = "p",
        long = "port",
        value_name = "PORT",
        default_value = "3030"
    )]
    pub register_service_port: u16,

    /// Specify the parse log file path
    #[structopt(
        long = "log",
        value_name = "PATH",
        default_value = "log/parse.log",
        parse(from_os_str)
    )]
    pub parse_log_path: PathBuf,

    /// Specify the roll size of parse log, unit: MB
    #[structopt(long = "roll-log-size", value_name = "SIZE", default_value = "500")]
    pub roll_log_size: u64,

    /// Specify the roll count of parse log
    #[structopt(long = "roll-log-count", value_name = "COUNT", default_value = "50")]
    pub roll_log_count: u32,

    /// Specify the sync log path
    #[cfg(feature = "sync-log")]
    #[structopt(
        long = "sync-log",
        value_name = "PATH",
        default_value = "log/sync.log",
        parse(from_os_str)
    )]
    pub sync_log_path: PathBuf,

    /// Recording sync log to parse log at INFO level
    #[cfg(feature = "sync-log")]
    #[structopt(long = "enable-sync-log")]
    pub enable_sync_log: bool,

    /// Specify the block height to start syncing
    #[cfg(feature = "sync-log")]
    #[structopt(long = "start-height", value_name = "HEIGHT", default_value = "0")]
    pub start_height: u64,

    /// Specify the block height to stop syncing
    #[cfg(feature = "sync-log")]
    #[structopt(long = "stop-height", value_name = "HEIGHT", default_value = "0")]
    pub stop_height: u64,

    /// Specify the url of redis server
    #[cfg(feature = "sync-redis")]
    #[structopt(
        long = "sync-redis",
        value_name = "URL",
        default_value = "redis://127.0.0.1"
    )]
    pub sync_redis_url: String,
}
