
#[macro_use]
extern crate logx;

use logx::{loggers::{global_logger::GlobaLoggerTrait, log_levels::LogLevel}, Logger, LogxTrait};




fn main() {
    // Set the default log level to Trace
    // all the log messages will be printed
    println!("Setting the default log level to Trace");
    Logger::set_log_level(LogLevel::Trace);

    log_error!("This is an error message");
    log_warn!("This is a warning message");
    log_info!("This is an info message");
    log_debug!("This is a debug message");
    log_trace!("This is a trace message");


    // Set the default log level to Info
    // This will override the previous default log level
    println!("Setting the default log level to Info");
    Logger::set_log_level(LogLevel::Info);

    log_error!("This is an error message");
    log_warn!("This is a warning message");
    log_info!("This is an info message");

    // below messages will not be printed as the log level is set to Info
    log_debug!("This is a debug message");
    log_trace!("This is a trace message");


    // lets set paronoia stile log  setting paranoia to true
    // wi will inherit the log level from the parent
    println!("Setting paranoia to true");
    Logger::set_paranoia(true);

    log_error!("This is an error message");
    log_warn!("This is a warning message");
    log_info!("This is an info message");

    // below messages will not be printed as the log level is set to Info
    log_debug!("This is a debug message");
    log_trace!("This is a trace message");
    


}