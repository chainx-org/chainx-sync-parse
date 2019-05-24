use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Duration, Utc};

/// This FileLogger rotates logs according to a time span.
/// After rotating, the original log file would be renamed to "{original name}.{%Y-%m-%d-%H}"
/// Note: log file will *not* be compressed or otherwise modified.
pub struct RotatingFileLogger {
    rotation_timespan: Duration,
    next_rotation_time: DateTime<Utc>,
    file_path: PathBuf,
    file: File,
}

impl Write for RotatingFileLogger {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.file.write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.should_rotate() {
            self.rotate()?;
        };
        self.file.flush()
    }
}

impl Drop for RotatingFileLogger {
    fn drop(&mut self) {
        let _ = self.file.flush();
    }
}

impl RotatingFileLogger {
    pub fn new(file_path: impl AsRef<Path>, rotation_timespan: Duration) -> io::Result<Self> {
        let file_path = file_path.as_ref().to_path_buf();
        let file = open_log_file(&file_path)?;
        let file_attr = fs::metadata(&file_path)?;
        let file_modified_time = file_attr.modified()?.into();
        let next_rotation_time = compute_rotation_time(file_modified_time, rotation_timespan);
        Ok(Self {
            next_rotation_time,
            file_path,
            rotation_timespan,
            file,
        })
    }

    fn should_rotate(&mut self) -> bool {
        Utc::now() > self.next_rotation_time
    }

    /// Rotates the current file and updates the next rotation time.
    fn rotate(&mut self) -> io::Result<()> {
        self.flush()?;

        // Note: renaming files while they're open only works on Linux and macOS.
        let new_path = rotation_file_path_with_timestamp(&self.file_path, &Utc::now());
        fs::rename(&self.file_path, new_path)?;
        let new_file = open_log_file(&self.file_path)?;
        self.update_rotation_time();
        self.file = new_file;
        Ok(())
    }

    /// Updates the next rotation time.
    fn update_rotation_time(&mut self) {
        let now = Utc::now();
        self.next_rotation_time = compute_rotation_time(now, self.rotation_timespan);
    }

    /// Flushes the log file, without rotation.
    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

/// Opens log file with append mode. Creates a new log file if it doesn't exist.
fn open_log_file(path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref();
    let parent = path
        .parent()
        .expect("Unable to get parent directory of log file");
    if !parent.is_dir() {
        fs::create_dir_all(parent)?
    }
    OpenOptions::new().append(true).create(true).open(path)
}

/// Adds `Duration` to the initial date and time.
fn compute_rotation_time(initial: DateTime<Utc>, timespan: Duration) -> DateTime<Utc> {
    initial + timespan
}

/// Rotates file path with given timestamp.
fn rotation_file_path_with_timestamp(
    file_path: impl AsRef<Path>,
    timestamp: &DateTime<Utc>,
) -> PathBuf {
    let mut file_path = file_path.as_ref().as_os_str().to_os_string();
    file_path.push(format!(".{}", timestamp.format("%Y-%m-%d-%H:%M:%S")));
    file_path.into()
}
