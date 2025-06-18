use std::fmt::Display;
use std::ffi::OsString;
use serde::{Deserialize, Serialize};
use crate::error::Error;

/// Enum representing available project languages
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum ProjectLanguage {
    C,
    CPP
}

impl ProjectLanguage {
    const C_STR: &'static str = "c";
    const CPP_STR: &'static str = "c++";
    const C_EXTENSIONS: [&'static str; 1] = ["c"];
    const CPP_EXTENSIONS: [&'static str; 2] = ["c", "cpp"];

    /// Returns an array of possible unit type variants as stings
    pub fn str_variants() -> [&'static str; 2] {
        [Self::C_STR, Self::CPP_STR]
    }

    /// Returns a vector containing possible source file extensions for the specific language
    pub fn extensions(&self) -> Vec<OsString> {
        match self {
            ProjectLanguage::C => Vec::from(Self::C_EXTENSIONS),
            ProjectLanguage::CPP => Vec::from(Self::CPP_EXTENSIONS)
        }.into_iter()
            .map(OsString::from)
            .collect()
    }
}

impl TryFrom<String> for ProjectLanguage {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            Self::C_STR => Ok(ProjectLanguage::C),
            Self::CPP_STR => Ok(ProjectLanguage::CPP),
            _ => Err(Error::EnumParseError(format!("Unexpected language value: {}", value)))
        }
    }
}

impl Into<String> for ProjectLanguage {
    fn into(self) -> String {
        match self {
            ProjectLanguage::C => Self::C_STR.to_string(),
            ProjectLanguage::CPP => Self::CPP_STR.to_string()
        }
    }
}

impl Display for ProjectLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ProjectLanguage::C => Self::C_STR.to_string(),
            ProjectLanguage::CPP => Self::CPP_STR.to_string()
        };
        write!(f, "{}", str)
    }
}