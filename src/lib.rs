//! # Logx Library
//!
//! The `logx` library provides a flexible and extensible logging framework for Rust applications.
//! It supports different log levels, module-specific logging, and customizable log output formats.
//!
//! ## Usage
//!
//! To use the `logx` library, you need to create log metadata and use the `Logger` to log messages. The library supports colorized output and paranoia mode for detailed logging.
//!
//! ```rust
//! use logx::loggers::log_levels::LogLevel;
//! use logx::Logger;
//! use logx::LogMetadata;
//!
//! let metadata = LogMetadata::new(
//!     "2023-10-01T12:00:00Z".to_string(),
//!     LogLevel::Info,
//!     "main.rs",
//!     "main".to_string(),
//!     42,
//!     "This is a log message".to_string(),
//! );
//!
//! Logger::log(&metadata);
//! ```
//! Or you can use the provided macros to log messages at different log levels.
//! ```rust
//!  log_error!("This is an error message");
//!  log_warn!("This is a warning message");
//!  log_info!("This is an info message");
//!  log_debug!("This is a debug message");
//!  log_trace!("This is a trace message");
//! ```
//!
//!
//!
//! ## Features
//!
//! - **Log Levels**: Supports multiple log levels (e.g., Error, Warn, Info, Debug, Trace).
//! - **Module Logging**: Allows setting log levels and paranoia mode for specific modules.
//! - **Colorized Output**: Supports colorizing log messages for better readability.
//! - **Paranoia Mode**: Provides detailed log output, including file, module, and line number information.
//! - **Flexible Configuration**: Allows customizing log levels and paranoia settings at runtime.
//!
//! ## Examples
//!
//! - ### Simple Example
//! The following example demonstrates how to use the `logx` library to log messages at different log levels.
//!
//! ```rust
//! fn main() {
//!     // Set the default log level to Trace
//!     // all the log messages will be printed
//!     Logger::set_log_level(LogLevel::Trace);
//!     println!("Setting the default log level to {}", Logger::get_log_level());
//!
//!     log_error!("This is an error message");
//!     log_warn!("This is a warning message");
//!     log_info!("This is an info message");
//!     log_debug!("This is a debug message");
//!     log_trace!("This is a trace message");
//!
//!     // Set the default log level to Info
//!     // This will override the previous default log level
//!     Logger::set_log_level(LogLevel::Info);
//!     println!("Setting the default log level to {}", Logger::get_log_level());
//!
//!     log_error!("This is an error message");
//!     log_warn!("This is a warning message");
//!     log_info!("This is an info message");
//!
//!     // below messages will not be printed as the log level is set to Info
//!     log_debug!("This is a debug message");
//!     log_trace!("This is a trace message");
//!
//!     // lets set paronoia stile log  setting paranoia to true
//!     // wi will inherit the log level from the parent
//!     Logger::set_paranoia(true);
//!     println!(
//!         "Setting paranoia to {}, this will inherit the log level from the parent, that is {}",
//!         Logger::get_paranoia(),
//!         Logger::get_log_level()
//!     );
//!
//!     log_error!("This is an error message");
//!     log_warn!("This is a warning message");
//!     log_info!("This is an info message");
//!
//!     // below messages will not be printed as the log level is set to Info
//!     log_debug!("This is a debug message");
//!     log_trace!("This is a trace message");
//! }
//! ```
//!
//! ### Output
//!
//! ```shell
//! Setting the default log level to TRACE
//!  [ 2024-10-05 10:44:56 - ERROR ] This is an error message
//!  [ 2024-10-05 10:44:56 - WARN  ] This is a warning message
//!  [ 2024-10-05 10:44:56 - INFO  ] This is an info message
//!  [ 2024-10-05 10:44:56 - DEBUG ] This is a debug message
//!  [ 2024-10-05 10:44:56 - TRACE ] This is a trace message
//!  Setting the default log level to INFO
//!  [ 2024-10-05 10:44:56 - ERROR ] This is an error message
//!  [ 2024-10-05 10:44:56 - WARN  ] This is a warning message
//!  [ 2024-10-05 10:44:56 - INFO  ] This is an info message
//!  Setting paranoia to true, this will inherit the log level from the parent, that is INFO
//!  [ 2024-10-05 10:44:56 - ERROR ] This is an error message | Target: simple | File: examples/simple.rs | Line: 44 |
//!  [ 2024-10-05 10:44:56 - WARN  ] This is a warning message | Target: simple | File: examples/simple.rs | Line: 45 |
//!  [ 2024-10-05 10:44:56 - INFO  ] This is an info message | Target: simple | File: examples/simple.rs | Line: 46 |
//! ```

//!
//! - ### Advanced Example per module logging
//! The following example demonstrates how to use the `logx` library to log messages at different log levels for different modules.
//!
//! ```rust
//! #[macro_use]
//! extern crate logx;
//!
//! use logx::{
//!     loggers::{ global_logger::{ DefaultLogLevel, DefaultLoggerTrait }, log_levels::LogLevel },
//!     Logger,
//!     LogxTrait,
//! };
//!
//! fn main() {
//!     // Set the default log level to Trace
//!     // all the log messages will be printed
//!
//!     println!("Setting the default log level to Debug");
//!     Logger::set_log_level(LogLevel::Debug);
//!
//!     println!("\n{:-<200}", "");
//!     println!("Logging from main, with log level of {}.\n", DefaultLogLevel::log_level());
//!     log_error!("This is an error message");
//!     log_warn!("This is a warning message");
//!     log_info!("This is an info message");
//!     log_debug!("This is a debug message");
//!     log_trace!("This is a trace message");
//!
//!     // calling the log_something function from mod_one
//!     mod_one::log_something();
//!
//!     // calling the log_something function from mod_two
//!     mod_two::log_something();
//!
//!     // calling the log_something function from mod_three
//!     mod_three::log_something();
//!
//!     // calling the log_something function from mod_four
//!     mod_four::log_something();
//! }
//!
//! mod mod_one {
//!     use logx::{
//!         loggers::{ log_levels::LogLevel, mod_logger::{ ModLogger, ModuleLoggerTrait } },
//!         Logger,
//!         LogxTrait,
//!     };
//!
//!     pub fn log_something() {
//!         let this_module = module_path!();
//!
//!         // setting the log level to Trace for this specific module mod_one
//!         Logger::set_mod_logging(this_module, LogLevel::Trace, false);
//!
//!         println!("\n{:-<200}", "");
//!         println!(
//!             "Logging from mod_one with log level of {}.\n",
//!             ModLogger::get_mod_log_level(module_path!()).unwrap()
//!         );
//!         log_error!("This is an error message from mod_one");
//!         log_warn!("This is a warning message from mod_one");
//!         log_info!("This is an info message from mod_one");
//!         log_debug!("This is a debug message from mod_one");
//!         log_trace!("This is a trace message from mod_one");
//!     }
//! }
//!
//! mod mod_two {
//!     use logx::{
//!         loggers::{ mod_logger::ModuleLoggerTrait, log_levels::LogLevel },
//!         Logger,
//!         LogxTrait,
//!     };
//!     pub fn log_something() {
//!         // setting the log level to Info for this specific module mod_two
//!         Logger::set_mod_logging(module_path!(), LogLevel::Info, false);
//!
//!         println!("\n{:-<200}", "");
//!         println!(
//!             "Logging from mod_two with log level of {}.\n",
//!             Logger::get_mod_log_level(module_path!()).unwrap()
//!         );
//!         log_error!("This is an error message from mod_two");
//!         log_warn!("This is a warning message from mod_two");
//!         log_info!("This is an info message from mod_two");
//!         log_debug!("This is a debug message from mod_two");
//!         log_trace!("This is a trace message from mod_two");
//!     }
//! }
//!
//! mod mod_three {
//!     use loggers::global_logger::DefaultLogLevel;
//!     use logx::*;
//!     pub fn log_something() {
//!         // using the default log level form the main function
//!
//!         println!("\n{:-<200}", "");
//!         println!(
//!             "Logging from mod_three with default log level form main with log level of {}.\n",
//!             DefaultLogLevel::log_level()
//!         );
//!         log_error!("This is an error message from mod_three");
//!         log_warn!("This is a warning message from mod_three");
//!         log_info!("This is an info message from mod_three");
//!         log_debug!("This is a debug message from mod_three");
//!         log_trace!("This is a trace message from mod_three");
//!     }
//! }
//!
//! mod mod_four {
//!     use logx::{
//!         loggers::{ mod_logger::ModuleLoggerTrait, log_levels::LogLevel },
//!         Logger,
//!         LogxTrait,
//!     };
//!
//!     pub fn log_something() {
//!         // setting the log level to Info for this specific module mod_four
//!         Logger::set_mod_logging(module_path!(), LogLevel::Warn, true);
//!
//!         println!("\n{:-<200}", "");
//!         println!(
//!             "Logging from mod_four with {} level and paranoia {} :P.\n",
//!             Logger::get_mod_log_level(module_path!()).unwrap(),
//!             Logger::get_mod_paranoia(module_path!())
//!         );
//!         log_error!("This is an error message from mod_three");
//!         log_warn!("This is a warning message from mod_three");
//!         log_info!("This is an info message from mod_three");
//!         log_debug!("This is a debug message from mod_three");
//!         log_trace!("This is a trace message from mod_three");
//!     }
//! }
//! ```
//! # Output
//! ```shell
//! Setting the default log level to Debug
//!
//! --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//! Logging from main, with log level of DEBUG.
//!
//! [ 2024-10-05 10:45:00 - ERROR ] This is an error message
//! [ 2024-10-05 10:45:00 - WARN  ] This is a warning message
//! [ 2024-10-05 10:45:00 - INFO  ] This is an info message
//! [ 2024-10-05 10:45:00 - DEBUG ] This is a debug message
//!
//! --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//! Logging from mod_one with log level of TRACE.
//!
//! [ 2024-10-05 10:45:00 - ERROR ] This is an error message from mod_one
//! [ 2024-10-05 10:45:00 - WARN  ] This is a warning message from mod_one
//! [ 2024-10-05 10:45:00 - INFO  ] This is an info message from mod_one
//! [ 2024-10-05 10:45:00 - DEBUG ] This is a debug message from mod_one
//! [ 2024-10-05 10:45:00 - TRACE ] This is a trace message from mod_one
//!
//! --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//! Logging from mod_two with log level of INFO.
//!
//! [ 2024-10-05 10:45:00 - ERROR ] This is an error message from mod_two
//! [ 2024-10-05 10:45:00 - WARN  ] This is a warning message from mod_two
//! [ 2024-10-05 10:45:00 - INFO  ] This is an info message from mod_two
//!
//! --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//! Logging from mod_three with default log level form main with log level of DEBUG.
//!
//! [ 2024-10-05 10:45:00 - ERROR ] This is an error message from mod_three
//! [ 2024-10-05 10:45:00 - WARN  ] This is a warning message from mod_three
//! [ 2024-10-05 10:45:00 - INFO  ] This is an info message from mod_three
//! [ 2024-10-05 10:45:00 - DEBUG ] This is a debug message from mod_three
//!
//! --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//! Logging from mod_four with WARN level and paranoia true :P.
//!
//! [ 2024-10-05 10:45:00 - ERROR ] This is an error message from mod_three | Target: multi_log_level::mod_four | File: examples/multi_log_level.rs | Line: 123 |
//! [ 2024-10-05 10:45:00 - WARN  ] This is a warning message from mod_three | Target: multi_log_level::mod_four | File: examples/multi_log_level.rs | Line: 124 |
//! ```
//!
//!
//! ## Modules
//!
//! - [`loggers`](loggers/index.html): Contains the core logging functionality, including global and module-specific loggers.
//! - [`terminal`](terminal/index.html): Provides utilities for terminal output, such as colorizing log messages.
//! - [`macros`](macros/index.html): Contains macros to simplify logging operations.
//!
//! ## Macros
//!
//! - [`log_error!`](macros/index.html#log_error): Logs an error message.
//! - [`log_warn!`](macros/index.html#log_warn): Logs a warning message.
//! - [`log_info!`](macros/index.html#log_info): Logs an informational message.
//! - [`log_debug!`](macros/index.html#log_debug): Logs a debug message.
//! - [`log_trace!`](macros/index.html#log_trace): Logs a trace message.
//! - [`timestamp!`](macros/index.html#timestamp): Generates a formatted timestamp string representing the current time.
//!
//!
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
        timestamp: String,
        level: LogLevel,
        file: &str,
        module: String,
        line: u32,
        message: String
    ) -> LogMetadata {
        LogMetadata {
            timestamp,
            level,
            file: file.to_string(),
            module,
            line,
            message,
        }
    }
    /// Returns the severity level of the log entry.
    pub fn level(&self) -> LogLevel {
        self.level.clone()
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

impl LogxTrait for Logger {
    /// Checks if logging is enabled for the given log metadata.
    fn enabled(metadata: &LogMetadata) -> bool {
        let module_log_level = ModLogger::get_mod_log_level(metadata.module.as_str());
        let default_level = DefaultLogLevel::log_level();
        if let Some(module_log_level) = module_log_level {
            return metadata.level <= module_log_level;
        }
        metadata.level <= default_level
    }
    /// Logs the given log metadata.
    fn log(metadata: &LogMetadata) {
        if Logger::enabled(metadata) {
            if Logger::enabled(metadata) {
                let timestamp = format!("{} - {}", metadata.timestamp(), metadata.level());
                let paranoia = format!(
                    " | Target: {} | File: {} | Line: {} | ",
                    metadata.module(),
                    metadata.file(),
                    metadata.line()
                );
                println!("[ {:<36} ] {}{}", timestamp, metadata.message(), if
                    DefaultLogLevel::paranoia() ||
                    ModLogger::get_mod_paranoia(metadata.module.as_str())
                {
                    paranoia.gray()
                } else {
                    "".to_string()
                });
            }
        }
    }
    /// Flushes the log output.
    fn flush() {
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
        ModLogger::set_log_level(module, log_level, paranoia);
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
