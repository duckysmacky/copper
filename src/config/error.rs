//! Project configuration specific error types and implementations

use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    /// The `copper.toml` configuration file was not found
    ProjectNotFound,
    /// Specified unit was not found in the list of the existing units
    UnitNotFound(String),
    /// The specified language doesn't exist
    InvalidLanguage(String),
    /// The specified compiler doesn't exist
    InvalidCompiler(String),
    /// The specified unit type doesn't exist
    InvalidUnitType(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ProjectNotFound => write!(f, "Copper project was not found. Create a new one with 'copper init'"),
            Error::UnitNotFound(s) => write!(f, "Unit '{}' was not found in project", s),
            Error::InvalidLanguage(s) => write!(f, "Invalid language value provided ('{}')", s),
            Error::InvalidCompiler(s) => write!(f, "Invalid compiler value provided ('{}')", s),
            Error::InvalidUnitType(s) => write!(f, "Invalid unit type value provided ('{}')", s),
        }
    }
}