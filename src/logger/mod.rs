mod formatter;
#[macro_use]
pub mod global;
pub mod rotate;

use std::fmt;
use std::io::{self, BufWriter};
use std::path::Path;
use std::sync::Mutex;

use chrono::Duration;
use slog::{Drain, Key, Level, OwnedKVList, Record, KV};
use slog_async::{Async, OverflowStrategy};
use slog_term::{Decorator, PlainDecorator, RecordDecorator, TermDecorator};

use self::rotate::RotatingFileLogger;

// Default is 128.
// Extended since blocking is set, and we don't want to block very often.
const SLOG_CHANNEL_SIZE: usize = 10240;
// Default is DropAndReport.
// It is not desirable to have dropped logs in our use case.
const SLOG_CHANNEL_OVERFLOW_STRATEGY: OverflowStrategy = OverflowStrategy::Block;
const TIMESTAMP_FORMAT: &str = "%Y/%m/%d %H:%M:%S%.3f %:z";

pub fn init_log<D>(drain: D, level: Level, use_async: bool)
where
    D: Drain + Send + 'static,
    <D as Drain>::Err: fmt::Display,
{
    let logger = if use_async {
        let drain = Async::new(LogAndFuse(drain))
            .chan_size(SLOG_CHANNEL_SIZE)
            .overflow_strategy(SLOG_CHANNEL_OVERFLOW_STRATEGY)
            .thread_name(thread_name("slogger"))
            .build()
            .filter_level(level)
            .fuse();
        slog::Logger::root(drain, slog::slog_o!())
    } else {
        let drain = LogAndFuse(Mutex::new(drain).filter_level(level));
        slog::Logger::root(drain, slog::slog_o!())
    };

    global::set_global(logger);
}

/// A simple alias to `PlainDecorator<BufWriter<RotatingFileLogger>>`.
// Avoid clippy type_complexity lint.
pub type RotatingFileDecorator = PlainDecorator<BufWriter<RotatingFileLogger>>;

/// Constructs a new file drainer which outputs log to a file at the specified
/// path. The file drainer rotates for the specified timespan.
pub fn file_drainer(
    path: impl AsRef<Path>,
    rotation_timespan: Duration,
) -> io::Result<LoggerFormat<RotatingFileDecorator>> {
    let logger = BufWriter::new(RotatingFileLogger::new(path, rotation_timespan)?);
    let decorator = PlainDecorator::new(logger);
    let drain = LoggerFormat::new(decorator);
    Ok(drain)
}

/// Constructs a new terminal drainer which outputs logs to stderr.
pub fn term_drainer() -> LoggerFormat<TermDecorator> {
    let decorator = TermDecorator::new().stderr().build();
    LoggerFormat::new(decorator)
}

pub struct LoggerFormat<D>
where
    D: Decorator,
{
    decorator: D,
}

impl<D> LoggerFormat<D>
where
    D: Decorator,
{
    pub fn new(decorator: D) -> Self {
        Self { decorator }
    }
}

impl<D> Drain for LoggerFormat<D>
where
    D: Decorator,
{
    type Ok = ();
    type Err = io::Error;

    fn log(&self, record: &Record<'_>, values: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        self.decorator.with_record(record, values, |decorator| {
            write_log_header(decorator, record)?;
            write_log_msg(decorator, record)?;
            write_log_fields(decorator, record, values)?;

            decorator.start_whitespace()?;
            writeln!(decorator)?;

            decorator.flush()?;

            Ok(())
        })
    }
}

struct LogAndFuse<D>(D);

impl<D> Drain for LogAndFuse<D>
where
    D: Drain,
    <D as Drain>::Err: std::fmt::Display,
{
    type Ok = ();
    type Err = slog::Never;

    fn log(&self, record: &Record<'_>, values: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        if let Err(e) = self.0.log(record, values) {
            let fatal_drainer = Mutex::new(term_drainer()).ignore_res();
            fatal_drainer.log(record, values).unwrap();
            let fatal_logger = slog::Logger::root(fatal_drainer, slog::slog_o!());
            slog::slog_crit!(
                fatal_logger,
                "logger encountered error, cannot continue working";
                "err" => %e,
            );
            panic!("logger encountered error");
        }
        Ok(())
    }
}

/// Writes log header to decorator.
/// Format: [date_time] [LEVEL] [source_file:line_number]
fn write_log_header(decorator: &mut dyn RecordDecorator, record: &Record<'_>) -> io::Result<()> {
    decorator.start_timestamp()?;
    write!(
        decorator,
        "[{}]",
        chrono::Local::now().format(TIMESTAMP_FORMAT)
    )?;

    decorator.start_whitespace()?;
    write!(decorator, " ")?;

    decorator.start_level()?;
    write!(decorator, "[{}]", get_unified_log_level(record.level()))?;

    decorator.start_whitespace()?;
    write!(decorator, " ")?;

    // Writes source file info.
    decorator.start_msg()?; // There is no `start_file()` or `start_line()`.
    if let Some(path) = Path::new(record.file())
        .file_name()
        .and_then(|path| path.to_str())
    {
        write!(decorator, "[")?;
        formatter::write_file_name(decorator, path)?;
        write!(decorator, ":{}]", record.line())?
    } else {
        write!(decorator, "[<unknown>]")?
    }

    Ok(())
}

/// Writes log message to decorator.
/// Format: [message]
/// Message must be a valid UTF-8 string and follows the same encoding rule for field key and field value.
fn write_log_msg(decorator: &mut dyn RecordDecorator, record: &Record<'_>) -> io::Result<()> {
    decorator.start_whitespace()?;
    write!(decorator, " ")?;

    decorator.start_msg()?;
    write!(decorator, "[")?;
    let msg = format!("{}", record.msg());
    formatter::write_escaped_str(decorator, &msg)?;
    write!(decorator, "]")?;

    Ok(())
}

/// Writes log fields to decorator.
/// Format: [field_key=field_value]
/// Log Field key and value must be valid UTF-8 strings.
/// If types other than string is provided, it must be converted to string.
/// If string in other encoding is provided, it must be converted to UTF-8.
fn write_log_fields(
    decorator: &mut dyn RecordDecorator,
    record: &Record<'_>,
    values: &OwnedKVList,
) -> io::Result<()> {
    let mut serializer = Serializer::new(decorator);
    record.kv().serialize(record, &mut serializer)?;
    values.serialize(record, &mut serializer)?;
    serializer.finish()?;
    Ok(())
}

struct Serializer<'a> {
    decorator: &'a mut dyn RecordDecorator,
}

impl<'a> Serializer<'a> {
    fn new(decorator: &'a mut dyn RecordDecorator) -> Self {
        Serializer { decorator }
    }

    fn write_whitespace(&mut self) -> io::Result<()> {
        self.decorator.start_whitespace()?;
        write!(self.decorator, " ")?;
        Ok(())
    }

    fn finish(self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> Drop for Serializer<'a> {
    fn drop(&mut self) {}
}

#[allow(clippy::write_literal)]
impl<'a> slog::ser::Serializer for Serializer<'a> {
    fn emit_none(&mut self, key: Key) -> slog::Result {
        self.emit_arguments(key, &format_args!("None"))
    }

    fn emit_arguments(&mut self, key: Key, val: &fmt::Arguments<'_>) -> slog::Result {
        self.write_whitespace()?;

        // Write key
        write!(self.decorator, "[")?;
        self.decorator.start_key()?;
        formatter::write_escaped_str(&mut self.decorator, key as &str)?;

        // Write separator
        self.decorator.start_separator()?;
        write!(self.decorator, "=")?;

        // Write value
        let value = format!("{}", val);
        self.decorator.start_value()?;
        formatter::write_escaped_str(self.decorator, &value)?;
        self.decorator.reset()?;
        write!(self.decorator, "]")?;
        Ok(())
    }
}

fn thread_name(name: &str) -> String {
    let res: Option<String> = std::thread::current()
        .name()
        .and_then(|name| name.split("::").skip(1).last())
        .map(From::from);
    res.map(|tag| format!("{}::{}", name, tag))
        .unwrap_or_else(|| name.to_owned())
}

// Converts `slog::Level` to unified log level format.
fn get_unified_log_level(lvl: Level) -> &'static str {
    match lvl {
        Level::Critical => "FATAL",
        Level::Error => "ERROR",
        Level::Warning => "WARN",
        Level::Info => "INFO",
        Level::Debug => "DEBUG",
        Level::Trace => "TRACE",
    }
}
