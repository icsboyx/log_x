#[macro_use]
extern crate logx;

use logx::{loggers::{global_logger::GlobaLoggerTrait, log_levels::LogLevel}, Logger, LogxTrait};

fn main() {
    // Set the default log level to Trace
    // all the log messages will be printed

    println!("Setting the default log level to Debug");
    Logger::set_log_level(LogLevel::Debug);

    println!("\n{:-<200}", "");
    println!("Logging from main.\n");
    log_error!("This is an error message");
    log_warn!("This is a warning message");
    log_info!("This is an info message");
    log_debug!("This is a debug message");
    log_trace!("This is a trace message");

    // calling the log_somthing function from mod_one
    mod_one::log_somthing();

    // calling the log_somthing function from mod_two
    mod_two::log_somthing();

    // calling the log_somthing function from mod_three
    mod_three::log_somthing();

    // calling the log_somthing function from mod_four
    mod_four::log_somthing();
}

mod mod_one {
    use logx::{loggers::{mod_logger::ModLoggerTriat, log_levels::LogLevel}, Logger, LogxTrait};

    pub fn log_somthing() {
        let this_module = module_path!();

        // setting the log level to Trace for this specific module mod_one
        Logger::set_mod_log_level(this_module, LogLevel::Trace, false);

        println!("\n{:-<200}", "");
        println!("Logging from mod_one with log level of Trace.\n");
        log_error!("This is an error message from mod_one");
        log_warn!("This is a warning message from mod_one");
        log_info!("This is an info message from mod_one");
        log_debug!("This is a debug message from mod_one");
        log_trace!("This is a trace message from mod_one");
    }
}

mod mod_two {
    use logx::{loggers::{mod_logger::ModLoggerTriat, log_levels::LogLevel}, Logger, LogxTrait};
    pub fn log_somthing() {
        // setting the log level to Info for this specific module mod_two
        Logger::set_mod_log_level(module_path!(), LogLevel::Trace, false);

        println!("\n{:-<200}", "");
        println!("Logging from mod_two with log level of Debug.\n");
        log_error!("This is an error message from mod_two");
        log_warn!("This is a warning message from mod_two");
        log_info!("This is an info message from mod_two");
        log_debug!("This is a debug message from mod_two");
        log_trace!("This is a trace message from mod_two");
    }
}

mod mod_three {
    use logx::*;
    pub fn log_somthing() {
        // using the default log level form the main function

        println!("\n{:-<200}", "");
        println!("Logging from mod_three with default log level form main.\n");
        log_error!("This is an error message from mod_three");
        log_warn!("This is a warning message from mod_three");
        log_info!("This is an info message from mod_three");
        log_debug!("This is a debug message from mod_three");
        log_trace!("This is a trace message from mod_three");
    }
}

mod mod_four {
    use logx::{loggers::{mod_logger::ModLoggerTriat, log_levels::LogLevel}, Logger, LogxTrait};
    
    pub fn log_somthing() {
        // setting the log level to Info for this specific module mod_four
        Logger::set_mod_log_level(module_path!(), LogLevel::Warn, true);

        println!("\n{:-<200}", "");
        println!("Logging from mod_four with warn level and paranoia :P.\n");
        log_error!("This is an error message from mod_three");
        log_warn!("This is a warning message from mod_three");
        log_info!("This is an info message from mod_three");
        log_debug!("This is a debug message from mod_three");
        log_trace!("This is a trace message from mod_three");
    }
}
