use std::{collections::HashMap, sync::{LazyLock, RwLock}};

use crate::log_levels::LogLevel;

// Define a global static variable for module-specific log levels
pub static MODULES_LOG_LEVEL: LazyLock<RwLock<HashMap<String, ModLogger>>> = LazyLock::new(||
    RwLock::new(HashMap::new())
);


pub trait ModLoggerTriat {
    fn set_mod_log_level(module: &str, log_level: LogLevel, paranoia: bool);
    fn mod_log_level(&self) -> LogLevel;
    fn mod_paranoia(&self) -> bool;
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ModLogger{
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
                eprintln!("Failed to set the log level for module {} in MODULES_LOG_LEVEL: {:?}", module, e);
            }
        }
    }

    pub fn log_level(&self) -> LogLevel {
        self.log_level.clone()  

    }

    pub fn paranoia(&self) -> bool {
        self.paranoia
    }
}





