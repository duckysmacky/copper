pub mod unit;
pub mod project;
mod error;

pub use error::{Error, Result};

/// Contains default values for project configuration
mod default {
    use std::path::PathBuf;
    /// Shortcut for generating callable closures from simple values so that Clap can use this value
    /// as default (it requires for a value to be generated from a closure)
    type Val<T> = fn() -> T;

    pub const BUILD_DIRECTORY_PATH: Val<PathBuf> = || PathBuf::from("build");
    pub const LOCAL_DIRECTORY_PATH: Val<PathBuf> = || PathBuf::from(".");
}

/// Contains closures to compare field values to their default values
mod equals {
    use super::default;
    use std::path::PathBuf;
    /// Shortcut for generating callable closures which compare a values with a default value
    type Eq<T> = fn(&T) -> bool;

    pub const BUILD_DIRECTORY_PATH: Eq<PathBuf> = |path| path.eq(&default::BUILD_DIRECTORY_PATH());
    pub const LOCAL_DIRECTORY_PATH: Eq<PathBuf> = |path| path.eq(&default::LOCAL_DIRECTORY_PATH());
}

pub const PROJECT_FILE_NAME: &str = "copper.toml";
#[allow(dead_code)]
pub const PROJECT_DIRECTORY_NAME: &str = ".copper";