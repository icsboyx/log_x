# log_x Library

The `log_x` library provides a flexible and extensible logging framework for Rust applications.
It supports different log levels, module-specific logging, and customizable log output formats, and destinations.
Log destination can be a file or a terminal. Each module can have its own log level and paranoia mode and destinations.
Logging simultaneously to a file and a terminal.


## Usage

To use the `log_x` library, you need to set the default log level and use the provided macros to log messages at different log levels.
Using embedded macros will automatically create log metadata for you.

You can also create metadata manually and log messages using the `Logger` API.

```rust
#[macro_use]
extern crate log_x;
use log_x::{ loggers::{ global_logger::DefaultLoggerTrait, log_levels::LogLevel }, Logger, LogMetadata };

fn main() {

  Logger::set_log_level(LogLevel::Trace);

   log_error!("This is an error message");
   log_warn!("This is a warning message");
   log_info!("This is an info message");
   log_debug!("This is a debug message");
   log_trace!("This is a trace message");


  let mut metadata = LogMetadata::new(
      "2023-10-01T12:00:00Z",
      LogLevel::Info,
      "main.rs",
      "main",
      42,
      "This is a log message",
  );

  Logger::log(&mut metadata);

}
```

## Features

- **Log Levels**: Supports multiple log levels (e.g., Error, Warn, Info, Debug, Trace).
- **Module Logging**: Allows setting log levels and paranoia mode for specific modules.
- **Colorized Output**: Supports colorizing log messages for better readability.
- **Paranoia Mode**: Provides detailed log output, including file and line number information.
- **Flexible Configuration**: Allows customizing log levels and paranoia settings at runtime.
- **Simple API**: Provides macros for logging messages at different log levels.

## Examples

- ### Simple Example
  The following example demonstrates how to use the `log_x` library to log messages at different log levels.

```rust
#[macro_use]
extern crate log_x;
use log_x::{ loggers::{ global_logger::DefaultLoggerTrait, log_levels::LogLevel }, Logger };

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

```shell
Setting the default log level to TRACE
[2024-10-08 02:24:37 - ERROR][simple] This is an error message
[2024-10-08 02:24:37 - WARN ][simple] This is a warning message
[2024-10-08 02:24:37 - INFO ][simple] This is an info message
[2024-10-08 02:24:37 - DEBUG][simple] This is a debug message
[2024-10-08 02:24:37 - TRACE][simple] This is a trace message
Setting the default log level to INFO
[2024-10-08 02:24:37 - ERROR][simple] This is an error message
[2024-10-08 02:24:37 - WARN ][simple] This is a warning message
[2024-10-08 02:24:37 - INFO ][simple] This is an info message
Setting paranoia to true, this will inherit the log level from the parent, that is INFO
[2024-10-08 02:24:37 - ERROR][simple] This is an error message | File: examples/simple.rs | Line: 40 |
[2024-10-08 02:24:37 - WARN ][simple] This is a warning message | File: examples/simple.rs | Line: 41 |
[2024-10-08 02:24:37 - INFO ][simple] This is an info message | File: examples/simple.rs | Line: 42 |

```

- ### Advanced Example per module logging
  The following example demonstrates how to use the `log_x` library to log messages at different log levels for different modules.

```rust
#[macro_use]
extern crate log_x;

use log_x::{
    loggers::{ global_logger::{ DefaultLogger, DefaultLoggerTrait }, log_levels::LogLevel },
    Logger,

};

fn main() {
    // Set the default log level to Trace
    // all the log messages will be printed

    println!("Setting the default log level to Debug");
    Logger::set_log_level(LogLevel::Debug);

    println!("\n{:-<200}", "");
    println!("Logging from main, with log level of {}.\n", DefaultLogger::log_level());
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
    use loggers::global_logger::DefaultLogger;
    use log_x::*;
    pub fn log_something() {
        // using the default log level form the main function

        println!("\n{:-<200}", "");
        println!(
            "Logging from mod_three with default log level form main with log level of {}.\n",
            DefaultLogger::log_level()
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
```

# Output

````shell
Setting the default log level to Debug

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Logging from main, with log level of DEBUG.

[2024-10-08 02:26:18 - ERROR][multi_log_level] This is an error message
[2024-10-08 02:26:18 - WARN ][multi_log_level] This is a warning message
[2024-10-08 02:26:18 - INFO ][multi_log_level] This is an info message
[2024-10-08 02:26:18 - DEBUG][multi_log_level] This is a debug message

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Logging from mod_one with log level of TRACE.

[2024-10-08 02:26:18 - ERROR][multi_log_level::mod_one] This is an error message from mod_one
[2024-10-08 02:26:18 - WARN ][multi_log_level::mod_one] This is a warning message from mod_one
[2024-10-08 02:26:18 - INFO ][multi_log_level::mod_one] This is an info message from mod_one
[2024-10-08 02:26:18 - DEBUG][multi_log_level::mod_one] This is a debug message from mod_one
[2024-10-08 02:26:18 - TRACE][multi_log_level::mod_one] This is a trace message from mod_one

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Logging from mod_two with log level of INFO.

[2024-10-08 02:26:18 - ERROR][multi_log_level::mod_two] This is an error message from mod_two
[2024-10-08 02:26:18 - WARN ][multi_log_level::mod_two] This is a warning message from mod_two
[2024-10-08 02:26:18 - INFO ][multi_log_level::mod_two] This is an info message from mod_two

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Logging from mod_three with default log level form main with log level of DEBUG.

[2024-10-08 02:26:18 - ERROR][multi_log_level::mod_three] This is an error message from mod_three
[2024-10-08 02:26:18 - WARN ][multi_log_level::mod_three] This is a warning message from mod_three
[2024-10-08 02:26:18 - INFO ][multi_log_level::mod_three] This is an info message from mod_three
[2024-10-08 02:26:18 - DEBUG][multi_log_level::mod_three] This is a debug message from mod_three

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Logging from mod_four with WARN level and paranoia true :P.

[2024-10-08 02:26:18 - ERROR][multi_log_level::mod_four] This is an error message from mod_three | File: examples/multi_log_level.rs | Line: 113 |
[2024-10-08 02:26:18 - WARN ][multi_log_level::mod_four] This is a warning message from mod_three | File: examples/multi_log_level.rs | Line: 114 |```

````

## Modules

- [`loggers`]: Contains the core logging functionality, including global and module-specific loggers.
- [`terminal`]: Provides utilities for terminal output, such as colorizing log messages.
- [`macros`]: Contains macros to simplify logging operations.

## Macros

- [`log_error!`]: Logs an error message.
- [`log_warn!`]: Logs a warning message.
- [`log_info!`]: Logs an informational message.
- [`log_debug!`]: Logs a debug message.
- [`log_trace!`]: Logs a trace message.
- [`timestamp!`]: Generates a formatted timestamp string representing the current time.
