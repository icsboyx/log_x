# LogX

This Rust library provides advanced logging functionality with customizable log levels, terminal colorization.

## Features

- **Customizable Log Levels**: Define and use different log levels.
- **Log levels can be changed**: You can change the log level at runtime.
- **Terminal Colorization**: Enhance log readability by colorizing terminal output.
- **Customizable Log Scope**: Define and use different log scopes: Global or Module-specific (Multiple modules can have their own log settings, or inherit form default).
- **Paranoia Mode**: Log Messages will print out also the: file name, line and module where the log message was called.
