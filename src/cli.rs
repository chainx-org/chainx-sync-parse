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
        default_value = "3030",
        parse(from_str = "parse_register_service_port")
    )]
    pub register_service_port: u16,

    /// Specify the log file path
    #[structopt(
        long = "log_file_path",
        default_value = "log/output.log",
        parse(from_os_str)
    )]
    pub log_file_path: PathBuf,

    /// Specify the url of redis server
    #[cfg(feature = "sync-redis")]
    #[structopt(long = "sync_redis_url", default_value = "redis://127.0.0.1")]
    pub sync_redis_url: String,

    /// Specify the sync log path
    #[cfg(feature = "sync-log")]
    #[structopt(long = "sync_log_path", default_value = "nohup.out")]
    pub sync_log_path: String,
}

fn parse_register_service_port(port: &str) -> u16 {
    port.parse::<u16>().unwrap()
}
