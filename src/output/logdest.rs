
/// Represents a logging destination, which can be stdout, a file, or both.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LogDestination {
    stdout: bool,
    file: Option<String>,
}

impl Default for LogDestination {
    /// Creates a default `LogDestination` that logs to stdout and not to a file.
    fn default() -> Self {
        LogDestination {
            stdout: true,
            file: None,
        }
    }
}

impl LogDestination {
    /// Creates a new `LogDestination` with the specified stdout and file settings.
    ///
    /// # Arguments
    ///
    /// * `stdout` - A boolean indicating whether to log to stdout.
    /// * `file` - An optional string specifying the file to log to.
    pub fn new(stdout: bool, file: Option<String>) -> Self {
        LogDestination { stdout, file }
    }

    /// Enables logging to stdout.
    pub fn log_to_stdout(&mut self) {
        self.stdout = true;
    }

    /// Sets the file to log to.
    ///
    /// # Arguments
    ///
    /// * `file` - A string specifying the file to log to.
    pub fn log_to_file(&mut self, file: String) {
        self.file = Some(file);
    }

    /// Disables logging to stdout.
    pub fn remove_stdout(&mut self) {
        self.stdout = false;
    }

    /// Removes the file logging destination.
    pub fn remove_file(&mut self) {
        self.file = None;
    }

    /// Disables all logging destinations.
    pub fn silent(&mut self) {
        self.stdout = false;
        self.file = None;
    }
}

/// Logs the given metadata to the appropriate destinations.
///
/// # Arguments
///
/// * `metadata` - A reference to the `LogMetadata` to be logged.
pub fn log_to_destination(metadata: &LogMetadata) {
    if metadata.log_destinations.stdout {
        log_to_stdout(metadata);
    }

    if let Some(file) = &metadata.log_destinations.file {
        log_to_file(metadata, file);
    }
}

/// Logs the given metadata to stdout.
///
/// # Arguments
///
/// * `metadata` - A reference to the `LogMetadata` to be logged.
pub fn log_to_stdout(metadata: &LogMetadata) {
    let timestamp = format!("{} - {}", metadata.timestamp(), metadata.level().colorized());
    let paranoia = format!(
        " | File: {} | Line: {} | ",
        metadata.file(),
        metadata.line()
    );

    let paranoia = match metadata.loggin_from_module {
        true => if ModLogger::get_mod_paranoia(metadata.module.as_str()) {
            paranoia
        } else {
            "".to_string()
        },
        false => if DefaultLogger::paranoia() {
            paranoia
        } else {
            "".to_string()
        }
    };

    println!("[{:^36}][{}] {}{}", timestamp, metadata.module().gray(), metadata.message(), paranoia);
}

/// Logs the given metadata to a file.
///
/// # Arguments
///
/// * `metadata` - A reference to the `LogMetadata` to be logged.
/// * `file` - The file to log to.
pub fn log_to_file(metadata: &LogMetadata, file: impl Into<String>) {
    let timestamp = format!("{} - {}", metadata.timestamp(), metadata.level());
    let paranoia = format!(
        " | File: {} | Line: {} | ",
        metadata.file(),
        metadata.line()
    );

    let paranoia = match metadata.loggin_from_module {
        true => if ModLogger::get_mod_paranoia(metadata.module.as_str()) {
            paranoia
        } else {
            "".to_string()
        },
        false => if DefaultLogger::paranoia() {
            paranoia
        } else {
            "".to_string()
        }
    };

    let payload = format!("[{:^27}][{}] {}{}", timestamp, metadata.module(), metadata.message(), paranoia);

    write_to_file(file, payload);
}

/// Writes a message to a file.
///
/// # Arguments
///
/// * `filename` - The name of the file to write to.
/// * `message` - The message to write to the file.
pub fn write_to_file(filename: impl Into<String>, message: impl Into<String>) {
    use std::fs::OpenOptions;
    use std::io::Write;
    let filename = filename.into();
    let file = Path::new(&filename);
    match OpenOptions::new().create(true).append(true).open(file) {
        Ok(mut file) => {
            writeln!(file, "{}", message.into()).unwrap();
        }
        Err(e) => {
            eprintln!("Error opening file: {} , {}", &file.display(), e);
        }
    }
}
