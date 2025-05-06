pub mod unit;
pub mod project;
mod error;

pub use error::{Error, Result};

/// Contains default values for project configuration
mod default {
    use std::path::PathBuf;
    type Val<T> = fn() -> T;

    pub const BUILD_DIRECTORY: Val<PathBuf> = || PathBuf::from("build/");
}

pub const PROJECT_FILE_NAME: &str = "copper.toml";
#[allow(dead_code)]
pub const PROJECT_DIRECTORY_NAME: &str = ".copper";