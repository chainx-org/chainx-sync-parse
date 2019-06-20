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
        long = "parse-log",
        value_name = "PATH",
        default_value = "log/parse.log",
        parse(from_os_str)
    )]
    pub parse_log_path: PathBuf,

    /// Specify the roll size of parse log, unit: MB
    #[structopt(long = "parse-roll-size", value_name = "SIZE", default_value = "200")]
    pub parse_roll_size: u64,

    /// Specify the roll count of parse log
    #[structopt(long = "parse-roll-count", value_name = "COUNT", default_value = "5")]
    pub parse_roll_count: u32,

    /// Specify the sync log path
    #[cfg(feature = "sync-log")]
    #[structopt(
        long = "sync-log",
        value_name = "PATH",
        default_value = "log/sync.log",
        parse(from_os_str)
    )]
    pub sync_log_path: PathBuf,

    /// Specify the starting block height to scan, range: [start,stop)
    #[cfg(feature = "sync-log")]
    #[structopt(long = "start-height", value_name = "HEIGHT", default_value = "0")]
    pub start_height: u64,

    /// Specify the stopping block height to scan
    #[cfg(feature = "sync-log")]
    #[structopt(
        long = "stop-height",
        value_name = "HEIGHT",
        default_value = "18446744073709551615"
    )]
    pub stop_height: u64,

    /// Specify the sync log rotate interval, unit: SECOND
    #[cfg(feature = "sync-log")]
    #[structopt(
        long = "log-rotate-interval",
        value_name = "SECOND",
        default_value = "30"
    )]
    pub log_rotate_interval: u32,

    /// Specify the url of redis server
    #[cfg(feature = "sync-redis")]
    #[structopt(
        long = "sync-redis",
        value_name = "URL",
        default_value = "redis://127.0.0.1"
    )]
    pub sync_redis_url: String,
}
