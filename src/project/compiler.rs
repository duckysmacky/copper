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
    const GCC_STR: &'static str = "gcc";
    const GPP_STR: &'static str = "g++";
    const CLANG_STR: &'static str = "clang";
    const MSVC_STR: &'static str = "msvc";
    
    pub fn str_variants() -> [&'static str; 4] {
        [Self::GCC_STR, Self::GPP_STR, Self::CLANG_STR, Self::MSVC_STR]
    }
    
    pub fn executable_name(&self) -> String {
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
        match value.to_lowercase().trim() {
            Self::GCC_STR => Ok(ProjectCompiler::GCC),
            Self::GPP_STR | "gpp" => Ok(ProjectCompiler::GPP),
            Self::CLANG_STR => Ok(ProjectCompiler::CLANG),
            Self::MSVC_STR | "cl" => Ok(ProjectCompiler::MSVC),
            _ => Err(Error::EnumParseError(format!("Unexpected compiler value: {}", value)))
        }
    }
}

impl Into<String> for ProjectCompiler {
    fn into(self) -> String {
        match self {
            ProjectCompiler::GCC => Self::GCC_STR.to_string(),
            ProjectCompiler::GPP => Self::GPP_STR.to_string(),
            ProjectCompiler::CLANG => Self::CLANG_STR.to_string(),
            ProjectCompiler::MSVC => Self::MSVC_STR.to_string()
        }
    }
}