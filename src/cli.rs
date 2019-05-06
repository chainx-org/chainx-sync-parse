use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "chainx-sync-parse",
    author = "ChainX <https://chainx.org>",
    about = "Synchronize and parse ChainX sync-node data"
)]
pub struct Cli {
    /// Specify the port of register service
    #[structopt(
        short = "p",
        long = "port",
        value_name = "PORT",
        default_value = "3030"
    )]
    pub register_service_port: u16,

    /// Specify the log file path
    #[structopt(
        long = "log-file-path",
        value_name = "PATH",
        default_value = "log/output.log",
        parse(from_os_str)
    )]
    pub log_file_path: PathBuf,

    /// Specify the url of redis server
    #[cfg(feature = "sync-redis")]
    #[structopt(
        long = "sync-redis",
        value_name = "URL",
        default_value = "redis://127.0.0.1"
    )]
    pub sync_redis_url: String,

    /// Specify the sync log path
    #[cfg(feature = "sync-log")]
    #[structopt(long = "sync-log", value_name = "PATH", default_value = "data/data.log")]
    pub sync_log_path: String,

    /// Specify the block height to start syncing
    #[cfg(feature = "sync-log")]
    #[structopt(long = "start-height", value_name = "HEIGHT", default_value = "0")]
    pub start_height: u64,
}
