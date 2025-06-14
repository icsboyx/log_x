//! This module provides functionality for managing module-specific log levels and paranoia settings.
//!
//! It defines a global static variable `MODULES_LOGGER` which is a thread-safe, lazily-initialized
//! `HashMap` that stores `ModLogger` instances for different modules. The `ModLogger` struct contains
//! the module name, log level, and paranoia flag.
//!
//! The [`ModuleLoggerTrait`](trait.ModuleLoggerTrait.html) trait defines the following methods for managing module-specific logging:
//!
//!
//! - `set_mod_logging(module: &str, log_level: LogLevel, paranoia: bool)`: Sets the log level and paranoia
//!   flag for a specific module.
//! - `get_mod_name(module: &str) -> String`: Retrieves the name of the module.
//! - `get_mod_log_level(module: &str) -> Option<LogLevel>`: Retrieves the log level for a specific module.
//! - `get_mod_paranoia(module: &str) -> bool`: Retrieves the paranoia flag for a specific module.
//!
//! The [`ModLogger`](struct.ModLogger.html) struct implements the `Default` trait, providing default values for its fields.
//!
//! The `ModLogger` struct also provides the following methods for managing module-specific logging:
//!
//! - `set_log_level(module: &str, log_level: LogLevel, paranoia: bool)`: Sets the log level and paranoia
//!   flag for a specific module and updates the `MODULES_LOGGER` map.
//! - `get_mod_name(module: &str) -> String`: Retrieves the name of the module from the `MODULES_LOGGER` map.
//! - `get_mod_log_level(module: &str) -> Option<LogLevel>`: Retrieves the log level for a specific module
//!   from the `MODULES_LOGGER` map.
//! - `get_mod_paranoia(module: &str) -> bool`: Retrieves the paranoia flag for a specific module from the
//!   `MODULES_LOGGER` map.
//!
//! Error handling is performed using `eprintln!` to print error messages if the read or write lock on
//! `MODULES_LOGGER` fails.
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

use super::log_levels::LogLevel;
use crate::output::logdest::LogDestination;

// Define a global static variable for module-specific log levels
/// A global static variable that holds module-specific log levels and paranoia settings.
pub static MODULES_LOGGER: LazyLock<RwLock<HashMap<String, ModLogger>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

/// A trait for managing module-specific log levels and paranoia settings.
pub trait ModuleLoggerTrait {
    /// Sets the log level and paranoia flag for a specific module.
    fn set_mod_logging(module: &str, log_level: LogLevel, paranoia: bool) {
        ModLogger::set_mod_log_level(module, log_level, paranoia);
    }
    /// Retrieves the name of the module.
    fn get_mod_name(module: &str) -> String {
        ModLogger::get_mod_name(module)
    }
    /// Retrieves the log level for a specific module. Returns `None` if the module is not found.
    fn get_mod_log_level(module: &str) -> Option<LogLevel> {
        ModLogger::get_mod_log_level(module)
    }
    /// Retrieves the paranoia flag for a specific module.
    fn get_mod_paranoia(module: &str) -> bool {
        ModLogger::get_mod_paranoia(module)
    }
    /// Log to file
    fn set_mod_log_to_file(module: &str, file: impl Into<String>) {
        ModLogger::set_mod_log_to_file(module, file.into());
    }

    /// Log to stdout
    fn set_mod_log_to_stdout(module: &str) {
        ModLogger::set_mod_log_to_stdout(module);
    }

    /// Remove file logging
    fn remove_mod_log_to_file(module: &str) {
        ModLogger::remove_mod_log_to_file(module);
    }

    /// Remove stdout logging
    fn remove_mod_log_stdout(module: &str) {
        ModLogger::remove_mod_log_stdout(module);
    }

    /// Silence logging
    fn set_mod_logging_silent(module: &str) {
        ModLogger::set_mod_logging_silent(module);
    }

    /// get log destination
    fn get_mod_log_destination(module: &str) -> Option<LogDestination> {
        ModLogger::get_mod_log_destination(module)
    }

    /// debug DEFAULT_LOGGER
    fn debug_mod_logger(module: &str) -> String {
        ModLogger::debug_mod_logger(module)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct ModLogger {
    /// The name of the module.
    pub module: String,
    /// The log level for the module.
    pub log_level: LogLevel,
    /// The paranoia flag for the module.
    pub paranoia: bool,
    /// The log destinations for the module.
    pub log_destinations: LogDestination,
}

impl ModLogger {
    /// Get the logging configuration for a module if exists
    pub fn get(module: &str) -> Option<ModLogger> {
        match MODULES_LOGGER.read() {
            Ok(modules_log_level) => match modules_log_level.get(module) {
                Some(mod_logger) => Some(mod_logger.clone()),
                None => None,
            },
            Err(e) => {
                eprintln!(
                    "Failed to get the log level for module {} in MODULES_LOGGER: {:?}",
                    module, e
                );
                None
            }
        }
    }

    /// Sets the log level and paranoia flag for a specific module.
    pub fn set_mod_log_level(module: &str, log_level: LogLevel, paranoia: bool) {
        match MODULES_LOGGER.write() {
            Ok(mut modules_log_level) => {
                modules_log_level.insert(
                    module.to_string(),
                    ModLogger {
                        module: module.to_string(),
                        log_level,
                        paranoia,
                        log_destinations: LogDestination::default(),
                    },
                );
            }
            Err(e) => {
                eprintln!(
                    "Failed to set the log level for module {} in MODULES_LOGGER: {:?}",
                    module, e
                );
            }
        }
    }

    /// Retrieves the name of the module.
    pub fn get_mod_name(module: &str) -> String {
        match MODULES_LOGGER.read() {
            Ok(modules_log_level) => match modules_log_level.get(module) {
                Some(mod_logger) => mod_logger.module.clone(),
                None => "".to_string(),
            },
            Err(e) => {
                eprintln!(
                    "Failed to get the log level for module {} in MODULES_LOGGER: {:?}",
                    module, e
                );
                "".to_string()
            }
        }
    }

    /// Retrieves the log level for a specific module. Returns `None` if the module is not found.
    pub fn get_mod_log_level(module: &str) -> Option<LogLevel> {
        match MODULES_LOGGER.read() {
            Ok(modules_log_level) => match modules_log_level.get(module) {
                Some(mod_logger) => Some(mod_logger.log_level.clone()),
                None => None,
            },
            Err(e) => {
                eprintln!(
                    "Failed to get the log level for module {} in MODULES_LOGGER: {:?}",
                    module, e
                );
                None
            }
        }
    }

    /// Retrieves the paranoia flag for a specific module.
    pub fn get_mod_paranoia(module: &str) -> bool {
        match MODULES_LOGGER.read() {
            Ok(modules_log_level) => match modules_log_level.get(module) {
                Some(mod_logger) => mod_logger.paranoia,
                None => false,
            },
            Err(e) => {
                eprintln!(
                    "Failed to get the paranoia for module {} in MODULES_LOGGER: {:?}",
                    module, e
                );
                false
            }
        }
    }

    // Log to file
    pub fn set_mod_log_to_file(module: &str, file: impl Into<String>) {
        match MODULES_LOGGER.write() {
            Ok(mut modules_log_level) => {
                if let Some(mod_logger) = modules_log_level.get_mut(module) {
                    mod_logger.log_destinations.log_to_file(file.into());
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to set the log destination for module {} in MODULES_LOGGER: {:?}",
                    module, e
                );
            }
        }
    }

    // Log to stdout
    pub fn set_mod_log_to_stdout(module: &str) {
        match MODULES_LOGGER.write() {
            Ok(mut modules_log_level) => {
                if let Some(mod_logger) = modules_log_level.get_mut(module) {
                    mod_logger.log_destinations.log_to_stdout();
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to set the log destination for module {} in MODULES_LOGGER: {:?}",
                    module, e
                );
            }
        }
    }

    // Remove file logging
    pub fn remove_mod_log_to_file(module: &str) {
        match MODULES_LOGGER.write() {
            Ok(mut modules_log_level) => {
                if let Some(mod_logger) = modules_log_level.get_mut(module) {
                    mod_logger.log_destinations.remove_file();
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to set the log destination for module {} in MODULES_LOGGER: {:?}",
                    module, e
                );
            }
        }
    }

    // Remove stdout logging
    pub fn remove_mod_log_stdout(module: &str) {
        match MODULES_LOGGER.write() {
            Ok(mut modules_log_level) => {
                if let Some(mod_logger) = modules_log_level.get_mut(module) {
                    mod_logger.log_destinations.remove_stdout();
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to set the log destination for module {} in MODULES_LOGGER: {:?}",
                    module, e
                );
            }
        }
    }

    // Silence logging
    pub fn set_mod_logging_silent(module: &str) {
        match MODULES_LOGGER.write() {
            Ok(mut modules_log_level) => {
                if let Some(mod_logger) = modules_log_level.get_mut(module) {
                    mod_logger.log_destinations.silent();
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to set the log destination for module {} in MODULES_LOGGER: {:?}",
                    module, e
                );
            }
        }
    }

    /// get log destination
    pub fn get_mod_log_destination(module: &str) -> Option<LogDestination> {
        match MODULES_LOGGER.read() {
            Ok(modules_log_level) => match modules_log_level.get(module) {
                Some(mod_logger) => Some(mod_logger.log_destinations.clone()),
                None => None,
            },
            Err(e) => {
                eprintln!(
                    "Failed to get the log destination for module {} in MODULES_LOGGER: {:?}",
                    module, e
                );
                None
            }
        }
    }

    /// debug DEFAULT_LOGGER
    pub fn debug_mod_logger(module: &str) -> String {
        match MODULES_LOGGER.read() {
            Ok(modules_log_level) => match modules_log_level.get(module) {
                Some(mod_logger) => format!("{:?}", mod_logger),
                None => "".to_string(),
            },
            Err(e) => {
                eprintln!(
                    "Failed to read the log level for module {} in MODULES_LOGGER: {:?}",
                    module, e
                );
                "".to_string()
            }
        }
    }
}
