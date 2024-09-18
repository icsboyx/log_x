// Import necessary items
pub mod loggers;
pub mod terminal;

#[macro_use]
pub mod macros;

use std::{ fmt::{ Debug, Display }, io::Write };

use loggers::{
    global_logger::{ DefaultLogLevel, DefaultLoggerTrait },
    log_levels::LogLevel,
    mod_logger::{ ModLogger, ModuleLoggerTrait },
};
use terminal::colors::Colorize;

// Implement the Colorize trait for all types that implement Display and Debug
impl<T: Display + Debug> Colorize for T {}

pub trait LogxTrait {
    fn enabled(metadata: &LogMetadata) -> bool;
    fn log(metadata: &LogMetadata);
    fn flush();
}

#[derive(Clone, Debug)]
pub struct LogMetadata {
    timestamp: String,
    level: LogLevel,
    file: String,
    module: String,
    line: u32,
    message: String,
}

impl LogMetadata {
    pub fn new(
        timestamp: String,
        level: LogLevel,
        file: &str,
        module: String,
        line: u32,
        message: String
    ) -> LogMetadata {
        LogMetadata {
            timestamp,
            level,
            file: file.to_string(),
            module,
            line,
            message,
        }
    }

    pub fn level(&self) -> LogLevel {
        self.level.clone()
    }

    pub fn module(&self) -> &str {
        &self.module
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
}

pub struct Logger {}

impl LogxTrait for Logger {
    fn enabled(metadata: &LogMetadata) -> bool {
        let module_log_level = ModLogger::get_mod_log_level(metadata.module.as_str());
        let default_level = DefaultLogLevel::log_level();
        if let Some(module_log_level) = module_log_level {
            return metadata.level <= module_log_level;
        }
        metadata.level <= default_level
    }

    fn log(metadata: &LogMetadata) {
        if Logger::enabled(metadata) {
            if Logger::enabled(metadata) {
                let timestamp = format!("{} - {}", metadata.timestamp(), metadata.level());
                let paranoia = format!(
                    " | Target: {} | File: {} | Line: {} | ",
                    metadata.module(),
                    metadata.file(),
                    metadata.line()
                );
                println!("[ {:<36} ] {}{}", timestamp, metadata.message(), if
                    DefaultLogLevel::paranoia() ||
                    ModLogger::get_mod_paranoia(metadata.module.as_str())
                {
                    paranoia.gray()
                } else {
                    "".to_string()
                });
            }
        }
    }
    fn flush() {
        match std::io::stdout().flush() {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to flush stdout: {:?}", e),
        }
    }
}

impl DefaultLoggerTrait for Logger {
    fn set_log_level(log_level: LogLevel) {
        DefaultLogLevel::set_log_level(log_level);
    }

    fn set_paranoia(paranoia: bool) {
        DefaultLogLevel::set_paranoia(paranoia);
    }

    fn get_log_level() -> LogLevel {
        DefaultLogLevel::log_level()
    }

    fn get_paranoia() -> bool {
        DefaultLogLevel::paranoia()
    }
}

impl ModuleLoggerTrait for Logger {
    fn set_mod_logging(module: &str, log_level: LogLevel, paranoia: bool) {
        ModLogger::set_log_level(module, log_level, paranoia);
    }

    fn get_mod_log_level(module: &str) -> Option<LogLevel> {
        ModLogger::get_mod_log_level(module)
    }

    fn get_mod_paranoia(module: &str) -> bool {
        ModLogger::get_mod_paranoia(module)
    }

    fn get_mod_name(module: &str) -> String {
        ModLogger::get_mod_name(module)
    }
}
