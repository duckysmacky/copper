#![allow(dead_code)]
use std::fmt::{Display, Formatter};
use std::process::Output;

/// Type alias for the custom error type
pub type Result<T> = std::result::Result<T, Error>;

/// Custom error type
#[derive(Debug)]
pub enum Error {
    /// Error related to configuration of the project
    ProjectConfigError(String),
    /// Error related to the Copper project in general
    ProjectError(String),
    /// Error related to the Copper unit in general
    UnitError(String),
    /// Error related to writing and reading files and directories 
    IOError(String),
    /// Error related to being unable to parse enum values
    EnumParseError(String),
}

// TODO: add better output for the Output
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ProjectConfigError(s) => write!(f, "Configuration error: {}", s),
            Error::ProjectError(s) => write!(f, "Project error: {}", s),
            Error::UnitError(s) => write!(f, "Project unit error: {}", s),
            Error::IOError(s) => write!(f, "IO error: {}", s),
            Error::EnumParseError(s) => write!(f, "Unable to parse an enum: {}", s),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        // TODO: match according to error kind for specific errors
        match error.kind() {
            _ => Error::IOError(error.to_string()),
        }
    }
}

/// Parses the output object and returns a formatted string containing exit code, stdout and stderr
pub fn parse_output(output: &Output) -> String {
    let mut message = String::new();
    message.push_str(format!("(Error code {})", output.status).as_str());

    if output.stdout.len() > 0 {
        message.push_str(format!("\nStdout:\n{}", String::from_utf8_lossy(&output.stdout)).as_str());
    }

    if output.stderr.len() > 0 {
        message.push_str(format!("\nStderr:\n{}", String::from_utf8_lossy(&output.stderr)).as_str());
    }

    message
}
