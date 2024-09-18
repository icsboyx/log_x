use std::{ fmt::Debug, sync::{ LazyLock, RwLock } };

use super::log_levels::LogLevel;

// Define global static variables for common log levels
pub static DEFAULT_LOG_LEVEL: LazyLock<RwLock<DefaultLogLevel>> = LazyLock::new(||
    RwLock::new(DefaultLogLevel::default())
);

pub trait DefaultLoggerTrait {
    fn set_log_level(log_level: LogLevel);
    fn set_paranoia(paranoia: bool);
    fn get_log_level() -> LogLevel;
    fn get_paranoia() -> bool;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DefaultLogLevel {
    pub default_log_level: LogLevel,
    pub paranoia: bool,
}

impl Default for DefaultLogLevel {
    fn default() -> Self {
        DefaultLogLevel {
            default_log_level: LogLevel::Off,
            paranoia: false,
        }
    }
}

impl DefaultLogLevel {
    pub fn set_log_level(log_level: LogLevel) {
        match DEFAULT_LOG_LEVEL.write() {
            Ok(mut default_log_level) => {
                default_log_level.default_log_level = log_level;
            }
            Err(e) => {
                eprintln!(
                    "Failed to set the default log level variable in DEFAULT_LOG_LEVEL: {:?}",
                    e
                );
            }
        }
    }

    pub fn set_paranoia(paranoia: bool) {
        match DEFAULT_LOG_LEVEL.write() {
            Ok(mut default_log_level) => {
                default_log_level.paranoia = paranoia;
            }
            Err(e) => {
                eprintln!("Failed to set the paranoia variable in DEFAULT_LOG_LEVEL: {:?}", e);
            }
        }
    }

    pub fn log_level() -> LogLevel {
        match DEFAULT_LOG_LEVEL.read() {
            Ok(default_log_level) => default_log_level.default_log_level.clone(),
            Err(e) => {
                eprintln!(
                    "Failed to read the default log level variable in DEFAULT_LOG_LEVEL: {:?}",
                    e
                );
                LogLevel::Off
            }
        }
    }

    pub fn paranoia() -> bool {
        match DEFAULT_LOG_LEVEL.read() {
            Ok(default_log_level) => default_log_level.paranoia,
            Err(e) => {
                eprintln!("Failed to read the paranoia variable in DEFAULT_LOG_LEVEL: {:?}", e);
                false
            }
        }
    }
}
