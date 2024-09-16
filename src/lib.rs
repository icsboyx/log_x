#![allow(dead_code)]

// Import necessary items
use std::{
    collections::HashMap,
    fmt::{ self, Debug, Display, Formatter },
    sync::{ LazyLock, RwLock },
};

// Define global static variables for default and module-specific log levels
pub static DEFAULT_LOG_LEVEL: LazyLock<RwLock<LogLevel>> = LazyLock::new(||
    RwLock::new(LogLevel::Off)
);
pub static MODULES_LOG_LEVEL: LazyLock<RwLock<HashMap<String, LogLevel>>> = LazyLock::new(||
    RwLock::new(HashMap::new())
);

// Define an enum to represent colors
enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Black,
    Reset,
}

// Implement a method to convert a Color to an ANSI code
impl Color {
    fn to_ansi_code(&self) -> &str {
        match self {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Black => "\x1b[30m",
            Color::Reset => "\x1b[0m",
        }
    }
}

// Define a trait that extends the Display and Debug trait with color methods
trait Colorize: Display + Debug {
    fn red(&self) -> String {
        format!("{}{}{}", Color::Red.to_ansi_code(), self, Color::Reset.to_ansi_code())
    }

    fn green(&self) -> String {
        format!("{}{}{}", Color::Green.to_ansi_code(), self, Color::Reset.to_ansi_code())
    }

    fn yellow(&self) -> String {
        format!("{}{}{}", Color::Yellow.to_ansi_code(), self, Color::Reset.to_ansi_code())
    }

    fn blue(&self) -> String {
        format!("{}{}{}", Color::Blue.to_ansi_code(), self, Color::Reset.to_ansi_code())
    }

    fn magenta(&self) -> String {
        format!("{}{}{}", Color::Magenta.to_ansi_code(), self, Color::Reset.to_ansi_code())
    }

    fn cyan(&self) -> String {
        format!("{}{}{}", Color::Cyan.to_ansi_code(), self, Color::Reset.to_ansi_code())
    }

    fn white(&self) -> String {
        format!("{}{}{}", Color::White.to_ansi_code(), self, Color::Reset.to_ansi_code())
    }

    fn black(&self) -> String {
        format!("{}{}{}", Color::Black.to_ansi_code(), self, Color::Reset.to_ansi_code())
    }
}

// Implement the Colorize trait for all types that implement Display and Debug
impl<T: Display + Debug> Colorize for T {}

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

#[derive(Clone)]
pub struct LogMetadata {
    timestamp: String,
    level: LogLevel,
    file: String,
    target: String,
    line: u32,
    message: String,
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
}

#[macro_export]
macro_rules! log_error {
    // Pattern for $target and $message without any additional arguments
    ($message:expr) => {
        logx::Logger::log(
            &logx::LogMetadata::new(
timestamp!(),
                logx::LogLevel::Error,
                file!(),
                module_path!().to_string(), 
                line!(),
                $message.to_string()
            )
        );
    };

    // Pattern for $target, $message, and additional format arguments
    (
        $message:expr,
        $($arg:tt)*
    ) => {
        logx::Logger::log(
            &logx::LogMetadata::new(
timestamp!(),
                logx::LogLevel::Error,
                file!(),
                module_path!().to_string(), 
                line!(),
                format!($message, $($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! log_warn {
    ($message:expr) => {
        logx::Logger::log(
            &logx::LogMetadata::new(
timestamp!(),
                logx::LogLevel::Warn,
                file!(),
                module_path!().to_string(), 
                line!(),
                $message.to_string()
            )
        );
    };

    (
        $target:expr,
        $message:expr,
        $($arg:tt)*
    ) => {
        logx::Logger::log(
            &logx::LogMetadata::new(
timestamp!(),
                logx::LogLevel::Warn,
                file!(),
                module_path!().to_string(), 
                line!(),
                format!($message, $($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! log_info {
    ($message:expr) => {
        logx::Logger::log(
            &logx::LogMetadata::new(
timestamp!(),
                logx::LogLevel::Info,
                file!(),
                module_path!().to_string(), 
                line!(),
                $message.to_string()
            )
        );
    };

    (
        $target:expr,
        $message:expr,
        $($arg:tt)*
    ) => {
        logx::Logger::log(
            &logx::LogMetadata::new(
timestamp!(),
                logx::LogLevel::Info,
                file!(),
                module_path!().to_string(), 
                line!(),
                format!($message, $($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! log_debug {
    ($message:expr) => {
        logx::Logger::log(
            &logx::LogMetadata::new(
timestamp!(),
                logx::LogLevel::Debug,
                file!(),
                module_path!().to_string(), 
                line!(),
                $message.to_string()
            )
        );
    };

    (
        $target:expr,
        $message:expr,
        $($arg:tt)*
    ) => {
        logx::Logger::log(
            &logx::LogMetadata::new(
timestamp!(),
                logx::LogLevel::Debug,
                file!(),
                module_path!().to_string(), 
                line!(),
                format!($message, $($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! log_trace {
    ($message:expr) => {
        logx::Logger::log(
            &logx::LogMetadata::new(
                timestamp!(),
                logx::LogLevel::Trace,
                file!(),
                module_path!().to_string(), 
                line!(),
                $message.to_string()
            )
        );
    };

    (
        $target:expr,
        $message:expr,
        $($arg:tt)*
    ) => {
        logx::Logger::log(
            &logx::LogMetadata::new(
timestamp!(),
                logx::LogLevel::Trace,
                file!(),
                module_path!().to_string(), 
                line!(),
                format!($message, $($arg)*)
            )
        );
    };
}

#[macro_export]
macro_rules! timestamp {
    () => {
        {
        // Import necessary items
        use std::time::{SystemTime, UNIX_EPOCH};
        use std::fmt::Write;

        // Get the current time since the Unix epoch
        let now = SystemTime::now();
        let duration_since_epoch = now.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        // Convert duration to seconds
        let seconds = duration_since_epoch.as_secs();

        // Constants for time conversion
        const SECONDS_IN_MINUTE: u64 = 60;
        const SECONDS_IN_HOUR: u64 = 3600;
        const SECONDS_IN_DAY: u64 = 86400;
        const DAYS_IN_YEAR: u64 = 365;

        // Convert seconds to a rough approximation of date components
        let years_since_epoch = seconds / (SECONDS_IN_DAY * DAYS_IN_YEAR);
        let year = 1970 + years_since_epoch as i32; // Start from Unix epoch (1970)

        let days_since_epoch = (seconds / SECONDS_IN_DAY) % DAYS_IN_YEAR;
        let month = 1 + (days_since_epoch / 30); // Approximate month (not accounting for leap years)
        let day = 1 + (days_since_epoch % 30);   // Approximate day

        // Convert seconds into hours, minutes, and seconds
        let hours = (seconds / SECONDS_IN_HOUR) % 24;  // Get hours in a 24-hour format
        let minutes = (seconds % SECONDS_IN_HOUR) / SECONDS_IN_MINUTE;
        let seconds = seconds % SECONDS_IN_MINUTE;

        // Format the timestamp
        let mut formatted_time = String::new();
        write!(formatted_time, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hours, minutes, seconds)
            .expect("Failed to format timestamp");

        formatted_time
        }
    };
}

pub struct Logger {
    default_log_level: LogLevel,
    modules_log_level: HashMap<String, LogLevel>,
}

impl Logger {
    pub fn set_default_log_level(log_level: LogLevel) {
        *DEFAULT_LOG_LEVEL.write().unwrap() = log_level;
    }

    pub fn set_module_log_level(module: &str, level: LogLevel) {
        MODULES_LOG_LEVEL.write().unwrap().insert(module.to_string(), level);
    }
    pub fn default_log_level() -> LogLevel {
        DEFAULT_LOG_LEVEL.read().unwrap().clone()
    }

    pub fn modules_log_level() -> HashMap<String, LogLevel> {
        MODULES_LOG_LEVEL.read().unwrap().clone()
    }

    pub fn enabled(metadata: &LogMetadata) -> bool {
        let module_level = MODULES_LOG_LEVEL.read().unwrap().get(metadata.target()).cloned();
        let default_level = DEFAULT_LOG_LEVEL.read().unwrap();

        if let Some(module_level) = module_level {
            metadata.level() >= module_level
        } else {
            metadata.level() >= *default_level
        }
    }

    pub fn log(metadata: &LogMetadata) {
        if Logger::enabled(metadata) {
            if Logger::enabled(metadata) {
                println!(
                    "[{} - {}] Target: {}, File: {}, Line: {} - {}",
                    metadata.timestamp(),
                    metadata.level(),
                    metadata.target(),
                    metadata.file(),
                    metadata.line(),
                    metadata.message()
                );
            }
        }
    }
}
