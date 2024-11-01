//! This module defines a global logger with adjustable log levels and paranoia settings.
//!
//! # Overview
//!
//! The `global_logger` module provides a global static logger that can be used to set and get
//! log levels and paranoia settings across the application. It uses a `LazyLock` to initialize
//! the logger lazily and an `RwLock` to ensure thread-safe read and write access to the logger's
//! state.
//!
//! # Structures
//!
//! - `DefaultLogger`: Represents the default log level and paranoia settings.
//!
//! # Traits
//!
//! - `DefaultLoggerTrait`: Defines the interface for setting and getting log levels and paranoia settings.
//!
//! # Static Variables
//!
//! - `DEFAULT_LOGGER`: A global static variable that holds the default log level and paranoia settings.
//!
//! # Functions
//! - [`DefaultLogger::set_log_level`](struct.DefaultLogger.html#method.set_log_level): Sets the global log level.
//! - [`DefaultLogger::set_paranoia`](struct.DefaultLogger.html#method.set_paranoia): Sets the global paranoia setting.
//! - [`DefaultLogger::get_log_level`](struct.DefaultLogger.html#method.get_log_level): Gets the global log level.
//! - [`DefaultLogger::get_paranoia`](struct.DefaultLogger.html#method.get_paranoia): Gets the global paranoia setting.
//!
//!
//! # Error Handling
//!
//! The functions that modify or read the global logger state handle potential errors by printing
//! error messages to the standard error output.
//!
//!

use std::{ fmt::Debug, sync::{ LazyLock, RwLock } };

use super::log_levels::LogLevel;

// Define global static variables for common log levels
pub static DEFAULT_LOGGER: LazyLock<RwLock<DefaultLogger>> = LazyLock::new(||
    RwLock::new(DefaultLogger::default())
);

pub trait DefaultLoggerTrait {
    /// Sets the global log level.
    fn set_log_level(log_level: LogLevel) {
        DefaultLogger::set_log_level(log_level);
    }
    /// Sets the global paranoia setting.
    fn set_paranoia(paranoia: bool) {
        DefaultLogger::set_paranoia(paranoia);
    }
    /// Gets the current global log level.
    fn get_log_level() -> LogLevel {
        DefaultLogger::log_level()
    }
    /// Gets the current global paranoia setting.
    fn get_paranoia() -> bool {
        DefaultLogger::paranoia()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DefaultLogger {
    pub default_log_level: LogLevel,
    pub paranoia: bool,
}

impl Default for DefaultLogger {
    /// Returns the default log level and paranoia settings.
    fn default() -> Self {
        DefaultLogger {
            default_log_level: LogLevel::Off,
            paranoia: false,
        }
    }
}

impl DefaultLogger {
    /// Sets the global log level.
    pub fn set_log_level(log_level: LogLevel) {
        match DEFAULT_LOGGER.write() {
            Ok(mut default_log_level) => {
                default_log_level.default_log_level = log_level;
            }
            Err(e) => {
                eprintln!("Failed to set the default log level variable in DEFAULT_LOGGER: {e}");
            }
        }
    }
    /// Sets the global paranoia setting.
    pub fn set_paranoia(paranoia: bool) {
        match DEFAULT_LOGGER.write() {
            Ok(mut default_log_level) => {
                default_log_level.paranoia = paranoia;
            }
            Err(e) => {
                eprintln!("Failed to set the paranoia variable in DEFAULT_LOGGER: {e}");
            }
        }
    }
    /// Gets the current global log level.
    pub fn log_level() -> LogLevel {
        match DEFAULT_LOGGER.read() {
            Ok(default_log_level) => default_log_level.default_log_level,
            Err(e) => {
                eprintln!("Failed to read the default log level variable in DEFAULT_LOGGER: {e}");
                LogLevel::Off
            }
        }
    }
    /// Gets the current global paranoia setting.
    pub fn paranoia() -> bool {
        match DEFAULT_LOGGER.read() {
            Ok(default_log_level) => default_log_level.paranoia,
            Err(e) => {
                eprintln!("Failed to read the paranoia variable in DEFAULT_LOGGER: {e}");
                false
            }
        }
    }
}
