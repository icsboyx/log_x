use std::{ collections::HashMap, sync::{ LazyLock, RwLock } };

use super::log_levels::LogLevel;

// Define a global static variable for module-specific log levels
pub static MODULES_LOG_LEVEL: LazyLock<RwLock<HashMap<String, ModLogger>>> = LazyLock::new(||
    RwLock::new(HashMap::new())
);

pub trait ModuleLoggerTrait {
    fn set_mod_logging(module: &str, log_level: LogLevel, paranoia: bool);
    fn get_mod_name(module: &str) -> String;
    fn get_mod_log_level(module: &str) -> Option<LogLevel>;
    fn get_mod_paranoia(module: &str) -> bool;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ModLogger {
    pub module: String,
    pub log_level: LogLevel,
    pub paranoia: bool,
}

impl Default for ModLogger {
    fn default() -> Self {
        ModLogger {
            module: "".to_string(),
            log_level: LogLevel::Off,
            paranoia: false,
        }
    }
}

impl ModLogger {
    pub fn set_log_level(module: &str, log_level: LogLevel, paranoia: bool) {
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
