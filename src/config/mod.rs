//! Contains types and functions for managing project configuration

mod unit;
mod project;
mod error;
mod language;
mod compiler;

pub use project::ProjectConfig;
pub use unit::{UnitConfig, UnitType};
pub use language::ProjectLanguage;
pub use compiler::ProjectCompiler;
pub use error::{Error, Result};

pub const PROJECT_FILE_NAME: &str = "copper.toml";
#[allow(dead_code)]
pub const PROJECT_DIRECTORY_NAME: &str = ".copper";

/// Contains default values for project configuration
pub mod default {
    #![allow(dead_code)]
    use std::path::PathBuf;
    /// Shortcut for generating callable closures from simple values so that Clap can use this value
    /// as default (it requires for a value to be generated from a closure)
    type Val<T> = fn() -> T;

    pub const LOCAL_DIRECTORY: Val<PathBuf> = || PathBuf::from(".");
    pub const BUILD_DIRECTORY: Val<PathBuf> = || PathBuf::from("build");
    pub const BINARY_DIRECTORY: Val<PathBuf> = || PathBuf::from("bin");
    pub const LIBRARY_DIRECTORY: Val<PathBuf> = || PathBuf::from("lib");
    pub const OBJECT_DIRECTORY: Val<PathBuf> = || PathBuf::from("obj");
    pub const SOURCE_DIRECTORY: Val<PathBuf> = || PathBuf::from("src");
}

/// Contains closures to compare field values to their default values
mod equals {
    #![allow(dead_code)]
    use super::default;
    use std::path::PathBuf;
    /// Shortcut for generating callable closures which compare a values with a default value
    type Eq<T> = fn(&T) -> bool;

    pub const LOCAL_DIRECTORY: Eq<PathBuf> = |path| path.eq(&default::LOCAL_DIRECTORY());
    pub const BUILD_DIRECTORY: Eq<PathBuf> = |path| path.eq(&default::BUILD_DIRECTORY());
    pub const BINARY_DIRECTORY: Eq<PathBuf> = |path| path.eq(&default::BINARY_DIRECTORY());
    pub const LIBRARY_DIRECTORY: Eq<PathBuf> = |path| path.eq(&default::LIBRARY_DIRECTORY());
    pub const OBJECT_DIRECTORY: Eq<PathBuf> = |path| path.eq(&default::OBJECT_DIRECTORY());
    pub const SOURCE_DIRECTORY: Eq<PathBuf> = |path| path.eq(&default::SOURCE_DIRECTORY());
}