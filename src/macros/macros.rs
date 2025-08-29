//! This module provides a set of logging macros for different log levels and a macro to generate a formatted timestamp.
//!
//! # Macros
//!
//! - `log_error!`: Logs an error message.
//! - `log_warn!`: Logs a warning message.
//! - `log_info!`: Logs an informational message.
//! - `log_debug!`: Logs a debug message.
//! - `log_trace!`: Logs a trace message.
//! - `timestamp!`: Generates a formatted timestamp string representing the current time.
//!
//! ## `log_error!` Macro
//!
//! Logs an error message with optional format arguments.
//!
//! ### Usage
//!
//! ```
//! use log_x::loggers::log_levels::LogLevel;
//! use log_x::{LogMetadata, Logger, log_error, timestamp};
//!
//! log_error!();
//! log_error!("An error occurred");
//! log_error!("Error: {}", timestamp!());
//! ```
//!
//! ## `log_warn!` Macro
//!
//! Logs a warning message with optional format arguments.
//!
//! ### Usage
//!
//! ```
//! use log_x::loggers::log_levels::LogLevel;
//! use log_x::{LogMetadata, Logger, log_warn, timestamp};
//!
//! log_warn!();
//! log_warn!("This is a warning");
//! log_warn!("Warning: {}", timestamp!());
//! ```
//!
//! ## `log_info!` Macro
//!
//! Logs an informational message with optional format arguments.
//!
//! ### Usage
//!
//! ```
//! use log_x::loggers::log_levels::LogLevel;
//! use log_x::{Logger, log_info, timestamp};
//!
//! log_info!();
//! log_info!("Informational message");
//! log_info!("Info: {}", timestamp!());
//! ```
//!
//! ## `log_debug!` Macro
//!
//! Logs a debug message with optional format arguments.
//!
//! ### Usage
//!
//! ```
//! use log_x::loggers::log_levels::LogLevel;
//! use log_x::{Logger, log_debug, timestamp};
//!
//! log_debug!();
//! log_debug!("Debugging message");
//! log_debug!("Debug: {}", timestamp!());
//! ```
//!
//! ## `log_trace!` Macro
//!
//! Logs a trace message with optional format arguments.
//!
//! ### Usage
//!
//! ```
//! use log_x::loggers::log_levels::LogLevel;
//! use log_x::{Logger, log_trace, timestamp};
//!
//! log_trace!();
//! log_trace!("Trace message");
//! log_trace!("Trace: {}", timestamp!());
//! ```
//!
//! ## `timestamp!` Macro
//!
//! Generates a formatted timestamp string representing the current time.
//!
//! ### Usage
//!
//! ```
//! use log_x::timestamp;
//! let current_timestamp = timestamp!();
//!
//! println!("Current Timestamp: {}", timestamp!());
//! ```
//!
//! ### Panics
//!
//! This macro will panic if the system time is before the Unix epoch (1970-01-01 00:00:00 UTC),
//! which would cause the duration calculation to fail.
//!
//!
#[macro_export]
macro_rules! log_error {
    // Print empty message for error log
    () => {
        log_x::Logger::log(
            &mut log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Error,
                file!(),
                module_path!().to_string(),
                line!(),
                "".to_string()
            )
        );
    };

    // Pattern for error log message with format arguments
    ($($arg:tt)*) => {
        log_x::Logger::log(
            &mut log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Error,
                file!(),
                module_path!().to_string(),
                line!(),
                format!($($arg)*)
            )
        );
    };
}
#[macro_export]
macro_rules! log_warn {
    // Print empty message for warning log
    () => {
        log_x::Logger::log(
            &mut log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Warn,
                file!(),
                module_path!().to_string(),
                line!(),
                "".to_string()
            )
        );
    };

    // Pattern for warning log message with format arguments
    ($($arg:tt)*) => {
        log_x::Logger::log(
            &mut log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Warn,
                file!(),
                module_path!().to_string(),
                line!(),
                format!($($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! log_info {
    // Print empty message for info log
    () => {
        log_x::Logger::log(
            &mut log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Info,
                file!(),
                module_path!().to_string(),
                line!(),
                "".to_string()
            )
        );
    };

    // Pattern for info log message with format arguments
    ($($arg:tt)*) => {
        log_x::Logger::log(
            &mut log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Info,
                file!(),
                module_path!().to_string(),
                line!(),
                format!($($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! log_debug {
    // Print empty message for debug log
    () => {
        log_x::Logger::log(
            &mut log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Debug,
                file!(),
                module_path!().to_string(),
                line!(),
                "".to_string()
            )
        );
    };

    // Pattern for debug log message with format arguments
    ($($arg:tt)*) => {
        log_x::Logger::log(
            &mut log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Debug,
                file!(),
                module_path!().to_string(),
                line!(),
                format!($($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! log_trace {
    // Print empty message for trace log
    () => {
        log_x::Logger::log(
            &mut log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Trace,
                file!(),
                module_path!().to_string(),
                line!(),
                "".to_string()
            )
        );
    };

    // Pattern for trace log message with format arguments
    ($($arg:tt)*) => {
        log_x::Logger::log(
            &mut log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Trace,
                file!(),
                module_path!().to_string(),
                line!(),
                format!($($arg)*)
            )
        );
    };
}

// Moving to chrono crate for better time handling,
// and cross-platform compatibility including  the ability to get local time

///
/// Local timestamp helper using `chrono::Local`.
///
/// # Usage
/// - `timestamp!()`                 – seconds precision (default)
/// - `timestamp!(milliseconds)`     – millisecond precision
/// - `timestamp!(millis)`           – alias for milliseconds
/// - `timestamp!(micro)` / `micros` – microsecond precision
/// - `timestamp!(nano)`  / `nanos`  – nanosecond precision
/// - `timestamp!("%Y-%m-%d %H:%M")` – custom chrono format (string literal)
///
/// # Examples
/// Basic/Default (seconds):
/// ```
/// // use log_x::timestamp; // uncomment and replace `your_crate` if you want this to compile
/// use log_x::timestamp;
/// println!("{}", timestamp!());
/// // Output (example): 2025-08-29 10:22:11
/// ```
///
/// Milliseconds:
/// ```
/// use log_x::timestamp;
/// println!("{}", timestamp!(milliseconds));
/// println!("{}", timestamp!(millis)); // alias
/// // Output (example): 2025-08-29 10:22:11.123
/// ```
///
/// Microseconds / Nanoseconds:
/// ```
/// use log_x::timestamp;
/// println!("{}", timestamp!(micro));  // Output (example): 2025-08-29 10:22:11.123456
/// println!("{}", timestamp!(nanos));  // Output (example): 2025-08-29 10:22:11.123456789
/// ```
///
/// Custom format (with timezone offset):
/// for format example and explanations see: <https://docs.rs/chrono/latest/chrono/format/strftime/index.html>
/// ```
/// use log_x::timestamp;
/// println!("{}", timestamp!("%Y-%m-%d %H:%M:%S %z"));
/// // Output (example): 2025-08-29 10:22:11 +0200
/// ```
#[macro_export]
macro_rules! timestamp {
    // default: seconds precision (local time)
    () => {{ ::chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string() }};

    // ---- specific shorthands FIRST ----
    (milliseconds) => {{ ::chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string() }};
    (millis) => {{ $crate::timestamp!(milliseconds) }};

    (micro) => {{ ::chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f").to_string() }};
    (micros) => {{ $crate::timestamp!(micro) }};

    (nano) => {{ ::chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.9f").to_string() }};
    (nanos) => {{ $crate::timestamp!(nano) }};

    // custom format string — keep LAST, limit to literals so identifiers don’t get captured
    ($fmt:literal) => {{ ::chrono::Local::now().format($fmt).to_string() }};

    // helpful error for anything else
    ($unknown:tt) => {
        compile_error!(
            "unsupported timestamp! argument. Use: milliseconds|millis|micro|micros|nano|nanos|\"custom fmt\""
        );
    };
}
