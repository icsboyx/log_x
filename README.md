# log_x

This Rust library provides advanced logging functionality with customizable log levels, terminal colorization.

## Features

- **Customizable Log Levels**: Define and use different log levels.
- **Log levels can be changed**: You can change the log level at runtime.
- **Terminal Colorization**: Enhance log readability by colorizing terminal output.
- **Customizable Log Scope**: Define and use different log scopes: Global or Module-specific (Multiple modules can have their own log settings, or inherit form default).
- **Paranoia Mode**: Log Messages will print out also the: file name, line and module where the log message was called.

To use the `log_x` library, you need to create log metadata and use the `Logger` to log messages. The library supports colorized output and paranoia mode for detailed logging.

```rust
use log_x::loggers::log_levels::LogLevel;
use log_x::Logger;
use log_x::LogMetadata;

let metadata = LogMetadata::new(
    "2023-10-01T12:00:00Z".to_string(),
    LogLevel::Info,
    "main.rs",
    "main".to_string(),
    42,
    "This is a log message".to_string(),
);

Logger::log(&metadata);
```

Or you can use the provided macros to log messages at different log levels.

```rust
 log_error!("This is an error message");
 log_warn!("This is a warning message");
 log_info!("This is an info message");
 log_debug!("This is a debug message");
 log_trace!("This is a trace message");
```

## Features

- **Log Levels**: Supports multiple log levels (e.g., Error, Warn, Info, Debug, Trace).
- **Module Logging**: Allows setting log levels and paranoia mode for specific modules.
- **Colorized Output**: Supports colorizing log messages for better readability.
- **Paranoia Mode**: Provides detailed log output, including file, module, and line number information.
- **Flexible Configuration**: Allows customizing log levels and paranoia settings at runtime.

## Examples

- ### Simple Example
  The following example demonstrates how to use the `log_x` library to log messages at different log levels.

```rust
fn main() {
    // Set the default log level to Trace
    // all the log messages will be printed
    Logger::set_log_level(LogLevel::Trace);
    println!("Setting the default log level to {}", Logger::get_log_level());

    log_error!("This is an error message");
    log_warn!("This is a warning message");
    log_info!("This is an info message");
    log_debug!("This is a debug message");
    log_trace!("This is a trace message");

    // Set the default log level to Info
    // This will override the previous default log level
    Logger::set_log_level(LogLevel::Info);
    println!("Setting the default log level to {}", Logger::get_log_level());

    log_error!("This is an error message");
    log_warn!("This is a warning message");
    log_info!("This is an info message");

    // below messages will not be printed as the log level is set to Info
    log_debug!("This is a debug message");
    log_trace!("This is a trace message");

    // lets set paronoia stile log  setting paranoia to true
    // wi will inherit the log level from the parent
    Logger::set_paranoia(true);
    println!(
        "Setting paranoia to {}, this will inherit the log level from the parent, that is {}",
        Logger::get_paranoia(),
        Logger::get_log_level()
    );

    log_error!("This is an error message");
    log_warn!("This is a warning message");
    log_info!("This is an info message");

    // below messages will not be printed as the log level is set to Info
    log_debug!("This is a debug message");
    log_trace!("This is a trace message");
}
```

### Output

````shell
Setting the default log level to TRACE
 [ 2024-10-05 10:44:56 - ERROR ] This is an error message
 [ 2024-10-05 10:44:56 - WARN  ] This is a warning message
 [ 2024-10-05 10:44:56 - INFO  ] This is an info message
 [ 2024-10-05 10:44:56 - DEBUG ] This is a debug message
 [ 2024-10-05 10:44:56 - TRACE ] This is a trace message
 Setting the default log level to INFO
 [ 2024-10-05 10:44:56 - ERROR ] This is an error message
 [ 2024-10-05 10:44:56 - WARN  ] This is a warning message
 [ 2024-10-05 10:44:56 - INFO  ] This is an info message
 Setting paranoia to true, this will inherit the log level from the parent, that is INFO
 [ 2024-10-05 10:44:56 - ERROR ] This is an error message | Target: simple | File: examples/simple.rs | Line: 44 |
 [ 2024-10-05 10:44:56 - WARN  ] This is a warning message | Target: simple | File: examples/simple.rs | Line: 45 |
 [ 2024-10-05 10:44:56 - INFO  ] This is an info message | Target: simple | File: examples/simple.rs | Line: 46 |
`

- ### Advanced Example per module logging
The following example demonstrates how to use the `log_x` library to log messages at different log levels for different modules.

```rust
#[macro_use]
extern crate log_x;

use log_x::{
    loggers::{ global_logger::{ DefaultLogLevel, DefaultLoggerTrait }, log_levels::LogLevel },
    Logger,
    log_xTrait,
};

fn main() {
    // Set the default log level to Trace
    // all the log messages will be printed

    println!("Setting the default log level to Debug");
    Logger::set_log_level(LogLevel::Debug);

    println!("\n{:-<200}", "");
    println!("Logging from main, with log level of {}.\n", DefaultLogLevel::log_level());
    log_error!("This is an error message");
    log_warn!("This is a warning message");
    log_info!("This is an info message");
    log_debug!("This is a debug message");
    log_trace!("This is a trace message");

    // calling the log_something function from mod_one
    mod_one::log_something();

    // calling the log_something function from mod_two
    mod_two::log_something();

    // calling the log_something function from mod_three
    mod_three::log_something();

    // calling the log_something function from mod_four
    mod_four::log_something();
}

mod mod_one {
    use log_x::{
        loggers::{ log_levels::LogLevel, mod_logger::{ ModLogger, ModuleLoggerTrait } },
        Logger,
        log_xTrait,
    };

    pub fn log_something() {
        let this_module = module_path!();

        // setting the log level to Trace for this specific module mod_one
        Logger::set_mod_logging(this_module, LogLevel::Trace, false);

        println!("\n{:-<200}", "");
        println!(
            "Logging from mod_one with log level of {}.\n",
            ModLogger::get_mod_log_level(module_path!()).unwrap()
        );
        log_error!("This is an error message from mod_one");
        log_warn!("This is a warning message from mod_one");
        log_info!("This is an info message from mod_one");
        log_debug!("This is a debug message from mod_one");
        log_trace!("This is a trace message from mod_one");
    }
}

mod mod_two {
    use log_x::{
        loggers::{ mod_logger::ModuleLoggerTrait, log_levels::LogLevel },
        Logger,
        log_xTrait,
    };
    pub fn log_something() {
        // setting the log level to Info for this specific module mod_two
        Logger::set_mod_logging(module_path!(), LogLevel::Info, false);

        println!("\n{:-<200}", "");
        println!(
            "Logging from mod_two with log level of {}.\n",
            Logger::get_mod_log_level(module_path!()).unwrap()
        );
        log_error!("This is an error message from mod_two");
        log_warn!("This is a warning message from mod_two");
        log_info!("This is an info message from mod_two");
        log_debug!("This is a debug message from mod_two");
        log_trace!("This is a trace message from mod_two");
    }
}

mod mod_three {
    use loggers::global_logger::DefaultLogLevel;
    use log_x::*;
    pub fn log_something() {
        // using the default log level form the main function

        println!("\n{:-<200}", "");
        println!(
            "Logging from mod_three with default log level form main with log level of {}.\n",
            DefaultLogLevel::log_level()
        );
        log_error!("This is an error message from mod_three");
        log_warn!("This is a warning message from mod_three");
        log_info!("This is an info message from mod_three");
        log_debug!("This is a debug message from mod_three");
        log_trace!("This is a trace message from mod_three");
    }
}

mod mod_four {
    use log_x::{
        loggers::{ mod_logger::ModuleLoggerTrait, log_levels::LogLevel },
        Logger,
        log_xTrait,
    };

    pub fn log_something() {
        // setting the log level to Info for this specific module mod_four
        Logger::set_mod_logging(module_path!(), LogLevel::Warn, true);

        println!("\n{:-<200}", "");
        println!(
            "Logging from mod_four with {} level and paranoia {} :P.\n",
            Logger::get_mod_log_level(module_path!()).unwrap(),
            Logger::get_mod_paranoia(module_path!())
        );
        log_error!("This is an error message from mod_three");
        log_warn!("This is a warning message from mod_three");
        log_info!("This is an info message from mod_three");
        log_debug!("This is a debug message from mod_three");
        log_trace!("This is a trace message from mod_three");
    }
}
````

# Output

```shell
Setting the default log level to Debug

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Logging from main, with log level of DEBUG.

[ 2024-10-05 10:45:00 - ERROR ] This is an error message
[ 2024-10-05 10:45:00 - WARN  ] This is a warning message
[ 2024-10-05 10:45:00 - INFO  ] This is an info message
[ 2024-10-05 10:45:00 - DEBUG ] This is a debug message

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Logging from mod_one with log level of TRACE.

[ 2024-10-05 10:45:00 - ERROR ] This is an error message from mod_one
[ 2024-10-05 10:45:00 - WARN  ] This is a warning message from mod_one
[ 2024-10-05 10:45:00 - INFO  ] This is an info message from mod_one
[ 2024-10-05 10:45:00 - DEBUG ] This is a debug message from mod_one
[ 2024-10-05 10:45:00 - TRACE ] This is a trace message from mod_one

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Logging from mod_two with log level of INFO.

[ 2024-10-05 10:45:00 - ERROR ] This is an error message from mod_two
[ 2024-10-05 10:45:00 - WARN  ] This is a warning message from mod_two
[ 2024-10-05 10:45:00 - INFO  ] This is an info message from mod_two

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Logging from mod_three with default log level form main with log level of DEBUG.

[ 2024-10-05 10:45:00 - ERROR ] This is an error message from mod_three
[ 2024-10-05 10:45:00 - WARN  ] This is a warning message from mod_three
[ 2024-10-05 10:45:00 - INFO  ] This is an info message from mod_three
[ 2024-10-05 10:45:00 - DEBUG ] This is a debug message from mod_three

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Logging from mod_four with WARN level and paranoia true :P.

[ 2024-10-05 10:45:00 - ERROR ] This is an error message from mod_three | Target: multi_log_level::mod_four | File: examples/multi_log_level.rs | Line: 123 |
[ 2024-10-05 10:45:00 - WARN  ] This is a warning message from mod_three | Target: multi_log_level::mod_four | File: examples/multi_log_level.rs | Line: 124 |
```

## Modules

- **loggers**: Contains the core logging functionality, including global and module-specific loggers.
- **terminal**: Provides utilities for terminal output, such as colorizing log messages.
- **macros**: Contains macros to simplify logging operations.

## Macros

- **log_error** Logs an error message.
- **log_warn!** Logs a warning message.
- **log_info!** Logs an informational message.
- **log_debug!** Logs a debug message.
- **log_trace!** Logs a trace message.
- **timestamp!** Generates a formatted timestamp string representing the current time.
