use serde::{Deserialize, Serialize};
use crate::error::Error;

/// Enum representing available project compilers
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum ProjectCompiler {
    GCC,
    GPP,
    CLANG,
    MSVC
}

impl ProjectCompiler {
    pub fn get_executable(&self) -> String {
        match self {
            ProjectCompiler::GCC => "gcc".to_string(),
            ProjectCompiler::GPP => "g++".to_string(),
            ProjectCompiler::CLANG => "clang".to_string(),
            ProjectCompiler::MSVC => "cl".to_string()
        }
    }
}

impl TryFrom<String> for ProjectCompiler {
    type Error = Error;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "gcc" => Ok(ProjectCompiler::GCC),
            "g++" => Ok(ProjectCompiler::GPP),
            "clang" => Ok(ProjectCompiler::CLANG),
            "msvc" => Ok(ProjectCompiler::MSVC),
            _ => Err(Error::EnumParseError(format!("Unexpected compiler value: {}", value)))
        }
    }
}

impl Into<String> for ProjectCompiler {
    fn into(self) -> String {
        match self {
            ProjectCompiler::GCC => "gcc".to_string(),
            ProjectCompiler::GPP => "g++".to_string(),
            ProjectCompiler::CLANG => "clang".to_string(),
            ProjectCompiler::MSVC => "msvc".to_string()
        }
    }
}