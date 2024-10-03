#![doc = include_str!("../README.md")]

// Import necessary items
pub mod loggers;
pub mod terminal;

#[macro_use]
pub mod macros;

use std::{ fmt::{ Debug, Display }, io::Write };

use loggers::{
    global_logger::{ DefaultLogLevel, DefaultLoggerTrait },
    log_levels::LogLevel,
    mod_logger::{ ModLogger, ModuleLoggerTrait },
};
use terminal::colors::Colorize;

// Implement the Colorize trait for all types that implement Display and Debug
/// A trait for colorizing log messages. For types that implement both `Display` and `Debug`.
impl<T: Display + Debug> Colorize for T {}

/// A trait for logging. Provides methods for checking if logging is enabled, logging messages, and flushing the log output.
pub trait LogxTrait {
    /// Checks if logging is enabled for the given log metadata.
    fn enabled(metadata: &LogMetadata) -> bool;
    /// Logs the given log metadata.
    fn log(metadata: &LogMetadata);
    /// Flushes the log output.
    fn flush();
}

#[derive(Clone, Debug)]
/// A structure representing log metadata.
pub struct LogMetadata {
    /// The timestamp when the log entry was created.
    timestamp: String,
    /// The severity level of the log entry.
    level: LogLevel,
    /// The file where the log entry was generated.
    file: String,
    /// The module where the log entry was generated.
    module: String,
    /// The line number in the file where the log entry was generated.
    line: u32,
    /// The log message.
    message: String,
}

/// A structure representing metadata for a log entry.
///
/// # Fields
/// - `timestamp`: The timestamp when the log entry was created.
/// - `level`: The severity level of the log entry.
/// - `file`: The file where the log entry was generated.
/// - `module`: The module where the log entry was generated.
/// - `line`: The line number in the file where the log entry was generated.
/// - `message`: The log message.
///
/// # Methods
/// - `new`: Creates a new `LogMetadata` instance.
/// - `level`: Returns the severity level of the log entry.
/// - `module`: Returns the module where the log entry was generated.
/// - `message`: Returns the log message.
/// - `file`: Returns the file where the log entry was generated.
/// - `line`: Returns the line number in the file where the log entry was generated.
/// - `timestamp`: Returns the timestamp when the log entry was created.

impl LogMetadata {
    /// Creates a new `LogMetadata` instance with the given values.
    pub fn new(
        timestamp: impl Into<String>,
        level: LogLevel,
        file: impl Into<String>,
        module: impl Into<String>,
        line: u32,
        message: impl Into<String>
    ) -> Self {
        Self {
            timestamp: timestamp.into(),
            level,
            file: file.into(),
            module: module.into(),
            line,
            message: message.into(),
        }
    }
    /// Returns the severity level of the log entry.
    pub fn level(&self) -> LogLevel {
        self.level
    }
    /// Returns the module where the log entry was generated.
    pub fn module(&self) -> &str {
        &self.module
    }
    /// Returns the log message.
    pub fn message(&self) -> &str {
        &self.message
    }
    /// Returns the file where the log entry was generated.
    pub fn file(&self) -> &str {
        &self.file
    }
    /// Returns the line number in the file where the log entry was generated.
    pub fn line(&self) -> u32 {
        self.line
    }
    /// Returns the timestamp when the log entry was created.
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
}

pub struct Logger {}

impl Logger {
    /// Checks if logging is enabled for the given log metadata.
    pub fn enabled(metadata: &LogMetadata) -> bool {
        let module_log_level = ModLogger::get_mod_log_level(metadata.module.as_str());
        let default_level = DefaultLogLevel::log_level();
        if let Some(module_log_level) = module_log_level {
            return metadata.level <= module_log_level;
        }
        metadata.level <= default_level
    }
    /// Logs the given log metadata.
    pub fn log(metadata: &LogMetadata) {
        if Logger::enabled(metadata) {
            let timestamp = format!("{} - {}", metadata.timestamp(), metadata.level());
            let paranoia = format!(
                " | File: {} | Line: {} | ",

                metadata.file(),
                metadata.line()
            );
            println!(
                "[{:^36}][{}] {}{}",
                timestamp,
                metadata.module().gray(),
                metadata.message(),
                if
                    DefaultLogLevel::paranoia() ||
                    ModLogger::get_mod_paranoia(metadata.module.as_str())
                {
                    paranoia.gray()
                } else {
                    "".to_string()
                }
            );
        }
    }
    /// Flushes the log output.
    pub fn flush() {
        match std::io::stdout().flush() {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to flush stdout: {:?}", e),
        }
    }
}

/// A trait for Default logging.
impl DefaultLoggerTrait for Logger {
    /// Sets the default log level.
    fn set_log_level(log_level: LogLevel) {
        DefaultLogLevel::set_log_level(log_level);
    }
    /// Sets the paranoia mode.
    fn set_paranoia(paranoia: bool) {
        DefaultLogLevel::set_paranoia(paranoia);
    }
    /// Returns the default log level.
    fn get_log_level() -> LogLevel {
        DefaultLogLevel::log_level()
    }
    /// Returns the paranoia mode.
    fn get_paranoia() -> bool {
        DefaultLogLevel::paranoia()
    }
}

/// A trait for Module logging.
impl ModuleLoggerTrait for Logger {
    /// Sets the log level for the given module.
    fn set_mod_logging(module: &str, log_level: LogLevel, paranoia: bool) {
        ModLogger::set_mod_log_level(module, log_level, paranoia);
    }
    /// Returns the log level for the given module. As an Option. If the module is not found, it will return None.
    fn get_mod_log_level(module: &str) -> Option<LogLevel> {
        ModLogger::get_mod_log_level(module)
    }
    /// Returns the paranoia mode for the given module.
    fn get_mod_paranoia(module: &str) -> bool {
        ModLogger::get_mod_paranoia(module)
    }
    /// Returns the name of the module.
    fn get_mod_name(module: &str) -> String {
        ModLogger::get_mod_name(module)
    }
}
