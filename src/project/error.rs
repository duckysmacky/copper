//! Project configuration specific error types and implementations

use std::fmt;
use crate::error;

/// A specialized error type for project configuration operations
pub type Error = error::CopperError<ErrorKind>;
/// A specialized `Result` type for project configuration operations
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ErrorKind {
    /// An unrecoverable I/O error occurred. The string contains more details about the error
    IOError(String),
    /// There is an error in the configuration file itself
    ConfigError,
    /// The project configuration file was not found
    ProjectNotFound,
    /// Project exists but is not accessible for some reason
    ProjectUnavailable,
    /// Specified unit was not found in the list of the existing units
    UnitNotFound(String),
    /// There are no source files in the unit
    NoSourceFiles,
    /// The specified language doesn't exist
    InvalidLanguage,
    /// The specified compiler doesn't exist
    InvalidCompiler,
    /// The specified unit type doesn't exist
    InvalidUnitType,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::IOError(msg) => write!(f, "{}", msg),
            ErrorKind::ConfigError => write!(f, "There is an error in the configuration file"),
            ErrorKind::ProjectNotFound => write!(f, "Copper project was not found in the current directory or any of its parents"),
            ErrorKind::ProjectUnavailable => write!(f, "Unable to access project"),
            ErrorKind::UnitNotFound(unit) => write!(f, "Unit '{}' was not found in project", unit),
            ErrorKind::InvalidLanguage => write!(f, "Invalid language value provided"),
            ErrorKind::InvalidCompiler => write!(f, "Invalid compiler value provided"),
            ErrorKind::InvalidUnitType => write!(f, "Invalid unit type value provided"),
            ErrorKind::NoSourceFiles => write!(f, "There are no source files in the unit"),
        }
    }
}

impl error::CopperErrorKind for ErrorKind {}