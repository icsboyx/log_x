//! This module provides functionality for terminal text coloring using ANSI escape codes.
//!
//! # Enums
//!
//! - `Color`: Represents various colors that can be used to colorize terminal text.
//!
//! # Traits
//!
//! - `Colorize`: Extends the `Display` and `Debug` traits with methods to colorize text.
//!
//! # Methods
//!
//! - `Color::to_ansi_code`: Converts a `Color` enum variant to its corresponding ANSI escape code.
//!
//! # Trait Methods
//!
//! The `Colorize` trait provides the following methods to colorize text:
//!
//! - `red`: Colors the text red.
//! - `green`: Colors the text green.
//! - `yellow`: Colors the text yellow.
//! - `blue`: Colors the text blue.
//! - `magenta`: Colors the text magenta.
//! - `cyan`: Colors the text cyan.
//! - `gray`: Colors the text gray.
//! - `white`: Colors the text white.
//! - `black`: Colors the text black.
//!
//! Each method returns a `String` with the text wrapped in the appropriate ANSI escape codes for the specified color.
use std::fmt::{ Display, Debug };

// Define an enum to represent colors
/// Represents various colors that can be used to colorize terminal text.
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    White,
    Black,
    Reset,
}

// Implement a method to convert a Color to an ANSI code
/// Converts a `Color` enum variant to its corresponding ANSI escape code.
impl Color {
    pub fn to_ansi_code(&self) -> &str {
        match self {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::Gray => "\x1b[90m",
            Color::White => "\x1b[37m",
            Color::Black => "\x1b[30m",
            Color::Reset => "\x1b[0m",
        }
    }
}

// Define a trait that extends the Display and Debug trait with color methods
/// Extends the `Display` and `Debug` traits with methods to colorize text.
/// Each method returns a `String` with the text wrapped in the appropriate ANSI escape codes for the specified color and resets the color at the end.
pub trait Colorize: Display + Debug {
    fn colorize(&self, color: Color) -> String {
        format!("{}{}{}", color.to_ansi_code(), self, Color::Reset.to_ansi_code())
    }

    fn red(&self) -> String {
        self.colorize(Color::Red)
    }

    fn green(&self) -> String {
        self.colorize(Color::Green)
    }

    fn yellow(&self) -> String {
        self.colorize(Color::Yellow)
    }

    fn blue(&self) -> String {
        self.colorize(Color::Blue)
    }

    fn magenta(&self) -> String {
        self.colorize(Color::Magenta)
    }

    fn cyan(&self) -> String {
        self.colorize(Color::Cyan)
    }

    fn gray(&self) -> String {
        self.colorize(Color::Gray)
    }

    fn white(&self) -> String {
        self.colorize(Color::White)
    }

    fn black(&self) -> String {
        self.colorize(Color::Black)
    }
}
