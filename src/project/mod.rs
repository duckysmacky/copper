//! Contains types and functions for managing project configuration

mod unit;
mod project;
mod error;
mod language;
mod compiler;
mod default;

pub use project::{CopperProject, ProjectConfig};
pub use unit::{UnitConfig, UnitType};
pub use language::ProjectLanguage;
pub use compiler::ProjectCompiler;
use error::{Result, Error, ErrorKind};

pub const PROJECT_FILE_NAME: &str = "copper.yaml";
#[allow(dead_code)]
pub const PROJECT_DIRECTORY_NAME: &str = ".copper";