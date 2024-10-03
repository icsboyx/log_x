//! This module defines the `LogLevel` enum and its associated implementations.
//!
//! The `LogLevel` enum represents various levels of logging severity, including:
//! - `Off`: No logging
//! - `Error`: Error messages
//! - `Warn`: Warning messages
//! - `Info`: Informational messages
//! - `Debug`: Debugging messages
//! - `Trace`: Trace messages
//!
//! The `LogLevel` enum derives several traits:
//! - `Clone`: Allows cloning of `LogLevel` values.
//! - `Debug`: Enables formatting of `LogLevel` values using the `{:?}` formatter.
//! - `PartialEq`: Allows comparison of `LogLevel` values for equality.
//! - `PartialOrd`: Allows comparison of `LogLevel` values for ordering.
//!
//! The `Default` trait is implemented for `LogLevel`, with the default value being `LogLevel::Off`.
//!
//! The `Display` trait is implemented for `LogLevel`, providing a way to format `LogLevel` values
//! as strings with associated colors. The colors are defined using the `Color` enum from the
//! `crate::terminal::colors` module.
//!
//! Additionally, the `LogLevel` enum provides a `from_str` method to create a `LogLevel` value
//! from a string representation. If the string does not match any known log level, `LogLevel::Off`
//! is returned.
use std::fmt::{ self, Display, Formatter };
use crate::terminal::colors::Color;

// Define an enum to represent log levels
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Default)]
/// Standard log levels for logging messages.
pub enum LogLevel {
    #[default]
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

// Implement the Display trait for LogLevel
/// Formats a `LogLevel` value as a string with associated colors. The colors are defined using the `Color` enum.
impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (color, level_str) = match self {
            LogLevel::Trace => (Color::Cyan, "TRACE"),
            LogLevel::Debug => (Color::Blue, "DEBUG"),
            LogLevel::Info => (Color::Green, "INFO"),
            LogLevel::Warn => (Color::Yellow, "WARN"),
            LogLevel::Error => (Color::Red, "ERROR"),
            LogLevel::Off => (Color::White, "OFF"),
        };

        // Write the colorized log level
        write!(f, "{}{}{}", color.to_ansi_code(), level_str, Color::Reset.to_ansi_code())
    }
}

/// Creates a `LogLevel` value from a string representation.
/// If the string does not match any known log level, `LogLevel::Off` is returned.
/// This function is used to parse log levels from configuration files or command-line arguments.

impl From<&str> for LogLevel {
    fn from(level: &str) -> Self {
        match level.to_uppercase().as_str() {
            "TRACE" => LogLevel::Trace,
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            "OFF" => LogLevel::Off,
            _ => LogLevel::Off,
        }
    }
}

impl From<String> for LogLevel {
    fn from(level: String) -> Self {
        level.as_str().into() // Delegate to the &str implementation
    }
}

impl Into<&'static str> for LogLevel {
    fn into(self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Off => "OFF",
        }
    }
}

impl Into<String> for LogLevel {
    fn into(self) -> String {
        self.to_string()
    }
}