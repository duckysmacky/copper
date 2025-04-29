use std::ffi::OsString;
use std::fmt::Display;
use std::string::String;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::vec;
use serde::{Deserialize, Serialize};
use crate::config::{default, PROJECT_FILE_NAME};
use crate::config::unit::{CopperUnit, UnitType};
use crate::error::{Error, Result};

/// Main Copper project configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct CopperProject {
    /// Name of the project
    name: String,
    /// Chosen language for the project
    pub(crate) language: ProjectLanguage,
    /// Chosen compiler for the project
    pub(crate) compiler: ProjectCompiler,
    /// Additional include paths for the whole project
    pub(crate) include_paths: Option<Vec<PathBuf>>,
    /// Unit configuration data
    #[serde(rename = "Unit")]
    units: Vec<CopperUnit>,
    /// Default build directory for all new units
    #[serde(default = "default::BUILD_DIRECTORY")]
    #[serde(skip_serializing)]
    default_build_directory: PathBuf,
    /// Location of the Copper project relative to where the command was executed.
    #[serde(skip)]
    pub(crate) project_location: PathBuf
}

impl CopperProject {
    /// Initialises the project with default values where possible and generates a new .toml file
    pub fn init(
        project_location: &Path,
        project_name: String,
        project_language: ProjectLanguage,
        generate_example: bool
    ) -> Result<PathBuf> {
        let default_compiler = if cfg!(target_os = "windows") {
            ProjectCompiler::MSVC
        } else {
            match &project_language {
                ProjectLanguage::C => ProjectCompiler::GCC,
                ProjectLanguage::CPP => ProjectCompiler::GPP
            }
        };

        let mut include_paths = None;
        let mut units = Vec::new();

        if generate_example {
            let src_dir = project_location.join("..");
            let build_dir = project_location.join("build");
            
            // TODO: add skip for 'already exists' io error
            fs::create_dir_all(src_dir.join("include"))?;
            fs::create_dir_all(build_dir.join("bin"))?;
            fs::create_dir_all(build_dir.join("obj"))?;

            include_paths = Some(vec![src_dir.join("include")]);

            units.push(CopperUnit::new(
                "example".to_string(),
                UnitType::Binary,
                PathBuf::from("src/"),
                PathBuf::from("build/bin"),
                PathBuf::from("build/obj")
            ));
        }

        let project = Self {
            name: project_name,
            language: project_language,
            compiler: default_compiler,
            include_paths,
            units,
            default_build_directory: default::BUILD_DIRECTORY(),
            project_location: project_location.to_path_buf()
        };

        let file_path = project_location.join(PROJECT_FILE_NAME);
        let mut file = File::create_new(&file_path)?;

        let toml_data = toml::to_string(&project)
            .map_err(|err| Error::ProjectConfigError(err.to_string()))?;

        file.write_all(toml_data.as_bytes())?;
        file.flush()?;
        Ok(file_path)
    }

    /// Imports a Copper project from a .toml project file
    pub fn import(directory: &Path) -> Result<Self> {
        let file_path = directory.join(PROJECT_FILE_NAME);
        let mut file = File::open(file_path)?;

        let mut file_data = String::new();
        file.read_to_string(&mut file_data)?;
        let mut project: CopperProject = toml::from_str(&file_data)
            .map_err(|err| Error::ProjectConfigError(err.to_string()))?;
        project.project_location = directory.to_path_buf();
        Ok(project)
    }

    /// Saves current Copper project to the .toml project file
    pub fn save(self, directory: &Path) -> Result<()> {
        let file_path = directory.join(PROJECT_FILE_NAME);
        let mut file = File::create(&file_path)?;

        let toml_data = toml::to_string(&self)
            .map_err(|err| Error::ProjectConfigError(err.to_string()))?;

        file.write_all(toml_data.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    /// Creates a new unit with minimum configuration and adds it to the project
    pub fn add_unit(&mut self, unit_name: String, unit_type: UnitType, unit_source: PathBuf) {
        let unit_type_directory = match &unit_type {
            UnitType::Binary => "bin/",
            UnitType::StaticLibrary => "lib/"
        };

        self.units.push(CopperUnit::new(
            unit_name,
            unit_type,
            unit_source,
            self.default_build_directory.join(unit_type_directory),
            self.default_build_directory.join("obj/")
        ))
    }

    /// Builds specifies units (by name) or the whole project
    pub fn build<'a>(&self, unit_names: Option<impl Iterator<Item = &'a String>>) -> Result<()> {
        if let Some(unit_names) = unit_names {
            for unit_name in unit_names {
                let unit = self.units.iter()
                    .find(|u| &u.name == unit_name);
                
                if let Some(unit) = unit {
                    unit.build(self)?;
                } else {
                    return Err(Error::UnitError(format!("Unknown unit '{}'", unit_name)))
                }
            }
        } else {
            self.units.iter()
                .try_for_each(|unit| unit.build(self))?;
        }

        Ok(())
    }
}

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
    pub fn get_strings() -> [&'static str; 2] {
        [
            Self::C_STR,
            Self::CPP_STR
        ]
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

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
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

/// Enum representing available project compilers
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum ProjectCompiler {
    GCC,
    GPP,
    CLANG,
    MSVC
}

impl TryFrom<String> for ProjectCompiler {
    type Error = Error;
    
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
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

