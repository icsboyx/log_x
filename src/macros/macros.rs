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
//! ```no_run
//! log_error!("An error occurred");
//! log_error!("Error: {}", error_message);
//! ```
//!
//! ## `log_warn!` Macro
//!
//! Logs a warning message with optional format arguments.
//!
//! ### Usage
//!
//! ```no_run
//! log_warn!("This is a warning");
//! log_warn!("Warning: {}", warning_message);
//! ```
//!
//! ## `log_info!` Macro
//!
//! Logs an informational message with optional format arguments.
//!
//! ### Usage
//!
//! ```no_run
//! log_info!("Informational message");
//! log_info!("Info: {}", info_message);
//! ```
//!
//! ## `log_debug!` Macro
//!
//! Logs a debug message with optional format arguments.
//!
//! ### Usage
//!
//! ```no_run
//! log_debug!("Debugging message");
//! log_debug!("Debug: {}", debug_message);
//! ```
//!
//! ## `log_trace!` Macro
//!
//! Logs a trace message with optional format arguments.
//!
//! ### Usage
//!
//! ```no_run
//! log_trace!("Trace message");
//! log_trace!("Trace: {}", trace_message);
//! ```
//!
//! ## `timestamp!` Macro
//!
//! Generates a formatted timestamp string representing the current time.
//!
//! ### Usage
//!
//! ```no_run
//! let current_timestamp = timestamp!();
//! println!("Current Timestamp: {}", current_timestamp);
//! ```
//!
//! ### Panics
//!
//! This macro will panic if the system time is before the Unix epoch (1970-01-01 00:00:00 UTC),
//! which would cause the duration calculation to fail.
#[macro_export]
macro_rules! log_error {
    // Pattern for $target and $message without any additional arguments
    ($message:expr) => {
        log_x::Logger::log(
            &log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Error,
                file!(),
                module_path!().to_string(), 
                line!(),
                $message.to_string()
            )
        );
    };

    // Pattern for $target, $message, and additional format arguments
    (
        $message:expr,
        $($arg:tt)*
    ) => {
        log_x::Logger::log(
            &log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Error,
                file!(),
                module_path!().to_string(), 
                line!(),
                format!($message, $($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! log_warn {
    ($message:expr) => {
        log_x::Logger::log(
            &log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Warn,
                file!(),
                module_path!().to_string(), 
                line!(),
                $message.to_string()
            )
        );
    };

    (
        $target:expr,
        $message:expr,
        $($arg:tt)*
    ) => {
        log_x::Logger::log(
            &log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Warn,
                file!(),
                module_path!().to_string(), 
                line!(),
                format!($message, $($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! log_info {
    ($message:expr) => {
        log_x::Logger::log(
            &log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Info,
                file!(),
                module_path!().to_string(), 
                line!(),
                $message.to_string()
            )
        );
    };

    (
        $target:expr,
        $message:expr,
        $($arg:tt)*
    ) => {
        log_x::Logger::log(
            &log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Info,
                file!(),
                module_path!().to_string(), 
                line!(),
                format!($message, $($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! log_debug {
    ($message:expr) => {
        log_x::Logger::log(
            &log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Debug,
                file!(),
                module_path!().to_string(), 
                line!(),
                $message.to_string()
            )
        );
    };

    (
        $target:expr,
        $message:expr,
        $($arg:tt)*
    ) => {
        log_x::Logger::log(
            &log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Debug,
                file!(),
                module_path!().to_string(), 
                line!(),
                format!($message, $($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! log_trace {
    ($message:expr) => {
        log_x::Logger::log(
            &log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Trace,
                file!(),
                module_path!().to_string(), 
                line!(),
                $message.to_string()
            )
        );
    };

    (
        $target:expr,
        $message:expr,
        $($arg:tt)*
    ) => {
        log_x::Logger::log(
            &log_x::LogMetadata::new(
                timestamp!(),
                log_x::loggers::log_levels::LogLevel::Trace,
                file!(),
                module_path!().to_string(), 
                line!(),
                format!($message, $($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! timestamp {
    () => {
        {
        // Import necessary items
        use std::time::{SystemTime, UNIX_EPOCH};
        use std::fmt::Write;

        // Get the current time since the Unix epoch
        let now = SystemTime::now();
        let duration_since_epoch = now.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        // Convert duration to seconds
        let seconds = duration_since_epoch.as_secs();

        // Constants for time conversion
        const DAYS_IN_YEAR: u64 = 365;
        const SECONDS_IN_MINUTE: u64 = 60;
        const SECONDS_IN_HOUR: u64 = 3600;
        const SECONDS_IN_DAY: u64 = 86400;


        // Convert seconds to a rough approximation of date components
        let years_since_epoch = seconds / (SECONDS_IN_DAY * DAYS_IN_YEAR);
        let year = 1970 + years_since_epoch as i32; // Start from Unix epoch (1970)

        let days_since_epoch = (seconds / SECONDS_IN_DAY) % DAYS_IN_YEAR;
        let month = 1 + (days_since_epoch / 30); // Approximate month (not accounting for leap years)
        let day = 1 + (days_since_epoch % 30);   // Approximate day

        // Convert seconds into hours, minutes, and seconds
        let hours = (seconds / SECONDS_IN_HOUR) % 24;  // Get hours in a 24-hour format
        let minutes = (seconds % SECONDS_IN_HOUR) / SECONDS_IN_MINUTE;
        let seconds = seconds % SECONDS_IN_MINUTE;

        // Format the timestamp
        let mut formatted_time = String::new();
        write!(formatted_time, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hours, minutes, seconds)
            .expect("Failed to format timestamp");

        formatted_time
        }
    };
}
