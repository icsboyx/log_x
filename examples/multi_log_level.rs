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
