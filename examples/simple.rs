extern crate logx;
use logx::*;


fn main() {
    Logger::set_default_log_level(LogLevel::Trace);
    log_error!("This is an error message");
    log_warn!("This is a warning message");
    log_info!("This is an info message");
    log_debug!("This is a debug message");
    log_trace!("This is a trace message");

    
}