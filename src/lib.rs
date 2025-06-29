#![doc = include_str!("../README.md")]

// Import necessary items
pub mod loggers;
pub mod output;
pub mod terminal;

#[macro_use]
pub mod macros;

use std::fmt::{Debug, Display};
use std::io::Write;

use loggers::global_logger::{DefaultLogger, DefaultLoggerTrait};
use loggers::log_levels::LogLevel;
use loggers::mod_logger::{ModLogger, ModuleLoggerTrait};
use output::logdest::{LogDestination, log_to_destination};
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
    /// logging from the module
    logging_from_module: bool,
    /// The line number in the file where the log entry was generated.
    line: u32,
    /// The log message.
    message: String,
    /// The log destinations.
    log_destinations: LogDestination,
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
        message: impl Into<String>,
    ) -> Self {
        Self {
            timestamp: timestamp.into(),
            level,
            file: file.into(),
            module: module.into(),
            logging_from_module: false,
            line,
            message: message.into(),
            log_destinations: LogDestination::default(),
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
    pub fn enabled(metadata: &mut LogMetadata) -> bool {
        let module_logger = ModLogger::get(metadata.module.as_str());
        let default_level = DefaultLogger::log_level();
        if let Some(module_logger) = module_logger {
            metadata.logging_from_module = true;
            metadata.log_destinations = module_logger.log_destinations;
            return metadata.level <= module_logger.log_level;
        }

        metadata.log_destinations = DefaultLogger::log_destination();
        metadata.level <= default_level
    }

    /// Logs the given log metadata.
    pub fn log(metadata: &mut LogMetadata) {
        if Logger::enabled(metadata) {
            log_to_destination(metadata);
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
impl DefaultLoggerTrait for Logger {}

/// A trait for Module logging.
impl ModuleLoggerTrait for Logger {}
