#[macro_use]
extern crate log_x;

use log_x::{ loggers::{ global_logger::DefaultLoggerTrait, log_levels::LogLevel }, Logger };

fn main() {
    // print the default settings for global logger
    println!("Defaultlogging settings are {}", Logger::display());

    // ############################################################################################################
    // Set loglevel destination to file
    // This will also log on stdout "default setting"
    // On error will not panic, it will print the error message and proceed
    println!("Setting log level to Error and logging to file");
    Logger::log_to_file("examples/gloabal_log.txt");

    // Set the default log level to Trace
    println!("Setting log level to Trace");
    Logger::set_log_level(LogLevel::Trace);
    println!("Logging settings are {}", Logger::display());

    // all the log messages will be printed
    log_error!("This is an error message");
    log_warn!("This is a warning message");
    log_info!("This is an info message");
    log_debug!("This is a debug message");
    log_trace!("This is a trace message");

    // ############################################################################################################
    // lets remove stdout logging
    println!("Removing stdout logging");
    Logger::remove_stdout();

    // Set the default log level to Info
    println!("Setting log level to Info");
    Logger::set_log_level(LogLevel::Info);
    
    // lets set paranoia
    println!("Setting paranoia");
    Logger::set_paranoia(true);
    
    println!("logging settings are {}", Logger::display());

    log_error!("This is an error message");
    log_warn!("This is a warning message");
    log_info!("This is an info message");

    // below messages will not be printed as the log level is set to Info
    log_debug!("This is a debug message");
    log_trace!("This is a trace message");

    // ############################################################################################################
    // lets set wrong file path
    // this will print the error message and proceed
    // lets add stdout logging
    // setting loggin level to trace
    println!("Readding stdout logging and setting log level to Trace and paranoia, and setting wrong file path");
    Logger::log_to_stdout();
    Logger::set_log_level(LogLevel::Trace);
    Logger::log_to_file("kkkkkkk/gloabal_log.txt");
    
    println!("logging settings are {}", Logger::display());

    // all the log messages will be printed
    log_error!("This is an error message");
    log_warn!("This is a warning message");
    log_info!("This is an info message");
    log_debug!("This is a debug message");
    log_trace!("This is a trace message");

    // ############################################################################################################
    // reset the log file path to correct path
    println!("Resetting the log file path to correct path \"examples/gloabal_log.txt\"");
    Logger::log_to_file("examples/gloabal_log.txt");

    println!("\n{:-<200}", "");

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
        // setting the log output to file
        Logger::set_mod_log_to_file(this_module, "examples/mod_one_log.txt");

        println!("\n{:-<200}", "");
        println!(
            "Logging from mod_one with log level of {}.\n",
            ModLogger::get_mod_log_level(module_path!()).unwrap()
        );
        println!("logging settings are {}\n", Logger::debug_mod_logger(this_module));

        // all the log messages will be printed
        log_error!("This is an error message from mod_one");
        log_warn!("This is a warning message from mod_one");
        log_info!("This is an info message from mod_one");
        log_debug!("This is a debug message from mod_one");
        log_trace!("This is a trace message from mod_one");
    }
}




mod mod_two {
    use log_x::{
        loggers::{ log_levels::LogLevel, mod_logger::{ ModLogger, ModuleLoggerTrait } },
        Logger,
    };

    pub fn log_something() {
        let this_module = module_path!();

        // setting the log level to Trace for this specific module mod_two
        Logger::set_mod_logging(this_module, LogLevel::Info, false);
        // setting the log output to file
        Logger::set_mod_log_to_file(this_module, "examples/mod_two_log.txt");

        println!("\n{:-<200}", "");
        println!(
            "Logging from mod_two with log level of {}.\n",
            ModLogger::get_mod_log_level(module_path!()).unwrap()
        );
        println!("logging settings are {}\n", Logger::debug_mod_logger(this_module));
        log_error!("This is an error message from mod_two");
        log_warn!("This is a warning message from mod_two");
        log_info!("This is an info message from mod_two");

        // below messages will not be printed as the log level is set to Info
        log_debug!("This is a debug message from mod_two");
        log_trace!("This is a trace message from mod_two");
    }
}


mod mod_three {
    use loggers::global_logger::{DefaultLogger, DefaultLoggerTrait};
    use log_x::*;
    pub fn log_something() {
        // using the default log level form the main function

        println!("\n{:-<200}", "");
        println!(
            "Logging from mod_three with default log level form main with log level of {}.\n",
            DefaultLogger::log_level()
        );
        println!("logging settings are {}\n", Logger::display());

        log_error!("This is an error message from mod_three");
        log_warn!("This is a warning message from mod_three");
        log_info!("This is an info message from mod_three");
        log_debug!("This is a debug message from mod_three");
        log_trace!("This is a trace message from mod_three");
    }
}

mod mod_four {
    use log_x::{ loggers::{log_levels::LogLevel, mod_logger::ModuleLoggerTrait }, Logger };

    pub fn log_something() {
        // setting the log level to warn for this specific module mod_four with paranoia true
        Logger::set_mod_logging(module_path!(), LogLevel::Warn, true);

        // setting the log output to file
        Logger::set_mod_log_to_file(module_path!(), "examples/mod_four_log.txt");



        // disabling the log output to stdout
        // only file logging will be done
        println!("\n{:-<200}", "");
        println!("Removing stdout logging");
        Logger::remove_mod_log_stdout(module_path!());

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