#![allow(dead_code)]

// Import necessary items
mod macros;
mod terminal;
pub mod log_levels;
pub mod global_logger;
pub mod mod_logger;


use global_logger::{DefaultLogLevel, GlobaLoggerTrait, DEFAULT_LOG_LEVEL};
use log_levels::LogLevel;
use mod_logger::{ ModLogger, ModLoggerTriat, MODULES_LOG_LEVEL};
use terminal:: Colorize;

use std::{
    fmt::{  Debug, Display }, fs::Metadata, io::{Stdout, Write}};

// Implement the Colorize trait for all types that implement Display and Debug
impl<T: Display + Debug> Colorize for T {}


pub trait Logx {
    fn enabled(metadata: &LogMetadata) -> bool;
    fn log(metadata: &LogMetadata);
    fn flush();
}

#[derive(Clone)]
pub struct LogMetadata {
    timestamp: String,
    level: LogLevel,
    file: String,
    target: String,
    line: u32,
    message: String,
    paranoia: bool,
}

impl LogMetadata {
    pub fn new(
        timestamp: String,
        level: LogLevel,
        file: &str,
        target: String,
        line: u32,
        message: String
    ) -> LogMetadata {
        LogMetadata {
            timestamp,
            level,
            file: file.to_string(),
            target,
            line,
            message,
            paranoia: false,
        }
    }

    pub fn level(&self) -> LogLevel {
        self.level.clone()
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn file(&self) -> &str {
        file!()
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }

    pub fn paranoia(&self) -> bool {
        self.paranoia
    }
}




pub struct Logger {}

impl Logx for Logger {
    fn enabled(metadata: &LogMetadata) -> bool {
        let module_level = MODULES_LOG_LEVEL.read().unwrap().get(metadata.target()).cloned();
        let default_level = DEFAULT_LOG_LEVEL.read().unwrap().default_log_level.clone();
        if let Some(
            module_level) = module_level {
            metadata.level() >= module_level.log_level()
        } else {
            metadata.level() >= default_level
        }
    }

    fn log(metadata: &LogMetadata) {
        if Logger::enabled(metadata) {
            if Logger::enabled(metadata) {
                let timestamp = format!("{} - {}", metadata.timestamp(), metadata.level());
                let paranioa = format!(" | Target: {} | File: {} | Line: {} | ",
                    metadata.target(),
                    metadata.file(),
                    metadata.line(),
                );
                println!(
                    "[ {:<36} ] {}{}",
                    timestamp,
                    metadata.message(),
                    if DefaultLogLevel::paranoia() { paranioa.gray() } else { "".to_string() }
                );
            }
        }
    }
    fn flush() {
        std::io::stdout().flush().unwrap();
    }
}


    impl GlobaLoggerTrait for Logger {
        fn set_log_level(log_level: LogLevel) {
            DefaultLogLevel::set_log_level(log_level);
        }

        fn set_paranoia(paranoia: bool) {
            DefaultLogLevel::set_paranoia(paranoia);
        }

        fn log_level() -> LogLevel {
            DefaultLogLevel::log_level()
        }

        fn paranoia() -> bool {
            DefaultLogLevel::paranoia()
        }
    }

impl ModLoggerTriat for Logger {
    fn set_mod_log_level(module: &str, log_level: LogLevel, paranoia: bool) {
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
                    module, e
                );
            }
        }
    }

    fn mod_log_level(&self) -> LogLevel {
        self.mod_log_level()
    }

    fn mod_paranoia(&self) -> bool {
        self.mod_paranoia()
    }
}