use log4rs::{
    append::{
        console,
        rolling_file::{self, policy},
    },
    config,
    encode::pattern::PatternEncoder,
};

use crate::cli::CliConfig;
use crate::error::Result;

const MB_SIZE: u64 = 1024 * 1024; // 1 MB

pub fn init(conf: &CliConfig) -> Result<()> {
    init_log4rs_with_config(&conf)?;
    version_info();
    info!(
        "Parse log [path: {:?}, roll size: {:?} MB, roll count: {:?}]",
        conf.parse_log_path, conf.parse_roll_size, conf.parse_roll_count
    );
    Ok(())
}

fn init_log4rs_with_config(conf: &CliConfig) -> Result<()> {
    let pattern = "{d(%Y-%m-%d %H:%M:%S)} {h({l})} - {m}\n";

    let console = console::ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    let trigger = policy::compound::trigger::size::SizeTrigger::new(conf.parse_roll_size * MB_SIZE);
    let roll_pattern = format!("{:?}.{{}}.gz", conf.parse_log_path);
    let roll = policy::compound::roll::fixed_window::FixedWindowRoller::builder()
        .build(roll_pattern.as_str(), conf.parse_roll_count)
        .expect("Building fixed window roller should't be fail");
    let policy = policy::compound::CompoundPolicy::new(Box::new(trigger), Box::new(roll));
    let roll_file = rolling_file::RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(&conf.parse_log_path, Box::new(policy))?;

    let log_config_builder = config::Config::builder()
        .appender(config::Appender::builder().build("console", Box::new(console)))
        .appender(config::Appender::builder().build("roll", Box::new(roll_file)));
    let root = config::Root::builder()
        .appender("console")
        .appender("roll")
        .build(log::LevelFilter::Info);
    let log_config = log_config_builder
        .build(root)
        .expect("Building log config shouldn't be fail");

    log4rs::init_config(log_config).expect("Initializing log config shouldn't be fail");
    Ok(())
}

fn version_info() {
    info!("================================================================================");
    info!(
        "Release Version:   {}",
        option_env!("CARGO_PKG_VERSION").unwrap_or("Unknown")
    );
    info!(
        "Git Commit Hash:   {}",
        option_env!("BUILD_GIT_HASH").unwrap_or("Unknown")
    );
    info!(
        "Git Commit Branch: {}",
        option_env!("BUILD_GIT_BRANCH").unwrap_or("Unknown")
    );
    info!(
        "Rust Version:      {}",
        option_env!("BUILD_RUSTC_VERSION").unwrap_or("Unknown")
    );
    info!("================================================================================");
}
