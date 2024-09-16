use crate::terminal::Color;
use std::fmt::{self, Display, Formatter};



// Define an enum to represent log levels
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Off
    }
}

// Implement the Display trait for LogLevel
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

impl LogLevel {
    pub fn from_str(level: &str) -> LogLevel {
        match level {
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