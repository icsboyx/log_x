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
