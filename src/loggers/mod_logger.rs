//! This module provides functionality for managing module-specific log levels and paranoia settings.
//!
//! It defines a global static variable `MODULES_LOG_LEVEL` which is a thread-safe, lazily-initialized
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
//!   flag for a specific module and updates the `MODULES_LOG_LEVEL` map.
//! - `get_mod_name(module: &str) -> String`: Retrieves the name of the module from the `MODULES_LOG_LEVEL` map.
//! - `get_mod_log_level(module: &str) -> Option<LogLevel>`: Retrieves the log level for a specific module
//!   from the `MODULES_LOG_LEVEL` map.
//! - `get_mod_paranoia(module: &str) -> bool`: Retrieves the paranoia flag for a specific module from the
//!   `MODULES_LOG_LEVEL` map.
//!
//! Error handling is performed using `eprintln!` to print error messages if the read or write lock on
//! `MODULES_LOG_LEVEL` fails.
use std::{ collections::HashMap, sync::{ LazyLock, RwLock } };

use super::log_levels::LogLevel;

// Define a global static variable for module-specific log levels
/// A global static variable that holds module-specific log levels and paranoia settings.
pub static MODULES_LOG_LEVEL: LazyLock<RwLock<HashMap<String, ModLogger>>> = LazyLock::new(||
    RwLock::new(HashMap::new())
);

/// A trait for managing module-specific log levels and paranoia settings.
pub trait ModuleLoggerTrait {
    /// Sets the log level and paranoia flag for a specific module.
    fn set_mod_logging(module: &str, log_level: LogLevel, paranoia: bool);
    /// Retrieves the name of the module.
    fn get_mod_name(module: &str) -> String;
    /// Retrieves the log level for a specific module. Returns `None` if the module is not found.
    fn get_mod_log_level(module: &str) -> Option<LogLevel>;
    /// Retrieves the paranoia flag for a specific module.
    fn get_mod_paranoia(module: &str) -> bool;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct ModLogger {
    /// The name of the module.
    pub module: String,
    /// The log level for the module.
    pub log_level: LogLevel,
    /// The paranoia flag for the module.
    pub paranoia: bool,
}



impl ModLogger {
    /// Sets the log level and paranoia flag for a specific module.
    pub fn set_mod_log_level(module: &str, log_level: LogLevel, paranoia: bool) {
        match MODULES_LOG_LEVEL.write() {
            Ok(mut modules_log_level) => {
                modules_log_level.insert(module.to_string(), ModLogger {
                    module: module.to_string(),
                    log_level,
                    paranoia,
                });
            }
            Err(e) => {
                eprintln!(
                    "Failed to set the log level for module {} in MODULES_LOG_LEVEL: {:?}",
                    module,
                    e
                );
            }
        }
    }

    /// Retrieves the name of the module.
    pub fn get_mod_name(module: &str) -> String {
        match MODULES_LOG_LEVEL.read() {
            Ok(modules_log_level) => {
                match modules_log_level.get(module) {
                    Some(mod_logger) => mod_logger.module.clone(),
                    None => "".to_string(),
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to get the log level for module {} in MODULES_LOG_LEVEL: {:?}",
                    module,
                    e
                );
                "".to_string()
            }
        }
    }

    /// Retrieves the log level for a specific module. Returns `None` if the module is not found.
    pub fn get_mod_log_level(module: &str) -> Option<LogLevel> {
        match MODULES_LOG_LEVEL.read() {
            Ok(modules_log_level) => {
                match modules_log_level.get(module) {
                    Some(mod_logger) => Some(mod_logger.log_level.clone()),
                    None => None,
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to get the log level for module {} in MODULES_LOG_LEVEL: {:?}",
                    module,
                    e
                );
                None
            }
        }
    }

    /// Retrieves the paranoia flag for a specific module.
    pub fn get_mod_paranoia(module: &str) -> bool {
        match MODULES_LOG_LEVEL.read() {
            Ok(modules_log_level) => {
                match modules_log_level.get(module) {
                    Some(mod_logger) => mod_logger.paranoia,
                    None => false,
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to get the paranoia for module {} in MODULES_LOG_LEVEL: {:?}",
                    module,
                    e
                );
                false
            }
        }
    }
}
