use std::ffi::OsString;
use std::string::String;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::compiler::{self, CompileOptions, Compiler};
use crate::error::Error;

pub const PROJECT_FILE_NAME: &str = "copper.toml";
#[allow(dead_code)]
pub const PROJECT_DIRECTORY_NAME: &str = ".copper";

/// Main Copper project configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct CopperProject {
    /// Name of the project
    name: String,
    /// Chosen language for the project
    language: Option<CopperProjectLanguage>,
    /// Chosen compiler for the project
    compiler: Option<CopperProjectCompiler>,
    /// Unit configuration data
    #[serde(rename = "Unit")]
    units: Option<Vec<CopperUnit>>,
    /// Location of the Copper project relative to where the command was executed.
    /// Usually kept as "." (for local directory)
    #[serde(skip)]
    project_location: PathBuf
}

/// Enum representing available project languages
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String", into = "String")]
enum CopperProjectLanguage {
    C,
    CPP
}

impl TryFrom<String> for CopperProjectLanguage {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "c" => Ok(CopperProjectLanguage::C),
            "cpp" => Ok(CopperProjectLanguage::CPP),
            _ => Err(Error::EnumParseError(format!("Unexpected language value: {}", value)))
        }
    }
}

impl Into<String> for CopperProjectLanguage {
    fn into(self) -> String {
        match self {
            CopperProjectLanguage::C => "c".to_string(),
            CopperProjectLanguage::CPP => "cpp".to_string()
        }
    }
}

impl CopperProjectLanguage {
    pub fn all_extensions(&self) -> Vec<OsString> {
        match self {
            CopperProjectLanguage::C => vec!["c", "h"]
                .into_iter()
                .map(OsString::from)
                .collect(),
            CopperProjectLanguage::CPP => vec!["c", "cpp", "h", "hpp"]
                .into_iter()
                .map(OsString::from)
                .collect(),
        }
    }
    
    pub fn source_extensions(&self) -> Vec<OsString> {
        match self {
            CopperProjectLanguage::C => vec!["c"]
                .into_iter()
                .map(OsString::from)
                .collect(),
            CopperProjectLanguage::CPP => vec!["c", "cpp"]
                .into_iter()
                .map(OsString::from)
                .collect(),
        }
    }
    
    pub fn header_extensions(&self) -> Vec<OsString> {
        match self {
            CopperProjectLanguage::C => vec!["h"]
                .into_iter()
                .map(OsString::from)
                .collect(),
            CopperProjectLanguage::CPP => vec!["h", "hpp"]
                .into_iter()
                .map(OsString::from)
                .collect(),
        }
    }
}

/// Enum representing available project compilers
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum CopperProjectCompiler {
    GCC,
    GPP,
    CLANG,
    MSVC
}

impl TryFrom<String> for CopperProjectCompiler {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "gcc" => Ok(CopperProjectCompiler::GCC),
            "g++" => Ok(CopperProjectCompiler::GPP),
            "clang" => Ok(CopperProjectCompiler::CLANG),
            "msvc" => Ok(CopperProjectCompiler::MSVC),
            _ => Err(Error::EnumParseError(format!("Unexpected compiler value: {}", value)))
        }
    }
}

impl Into<String> for CopperProjectCompiler {
    fn into(self) -> String {
        match self {
            CopperProjectCompiler::GCC => "gcc".to_string(),
            CopperProjectCompiler::GPP => "g++".to_string(),
            CopperProjectCompiler::CLANG => "clang".to_string(),
            CopperProjectCompiler::MSVC => "msvc".to_string()
        }
    }
}

/// Configuration for the unit
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CopperUnit {
    /// Name of the unit
    name: String,
    /// Type of the unit
    r#type: UnitType,
    /// Location of the unit within the project
    source: PathBuf,
    /// Per-unit build output location
    output_directory: PathBuf,
    /// Per-unit location for intermediate files
    intermediate_directory: PathBuf,
}

/// Enum representing available project languages
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum UnitType {
    Binary,
    StaticLibrary
}

impl TryFrom<String> for UnitType {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "binary" => Ok(UnitType::Binary),
            "static_library" => Ok(UnitType::StaticLibrary),
            _ => Err(Error::EnumParseError(format!("Unexpected unit type value: {}", value)))
        }
    }
}

impl Into<String> for UnitType {
    fn into(self) -> String {
        match self {
            UnitType::Binary => "binary".to_string(),
            UnitType::StaticLibrary => "static_library".to_string()
        }
    }
}

impl CopperUnit {
    pub fn build(&self, project: &CopperProject) {
        let unit_path = project.project_location.join(&self.source);
        let unit_dir = fs::read_dir(&unit_path).expect("Unit location not found");

        let language = project.language.clone().expect("Project language not selected");

        let mut source_file_paths: Vec<PathBuf> = Vec::new();
        for entry in unit_dir {
            let file_path = entry.unwrap().path();

            if let Some(ext) = file_path.extension() {
                if language.source_extensions().contains(&ext.to_os_string()) {
                    source_file_paths.push(file_path);
                }
            }
        }

        let mut output_dir = PathBuf::from(&project.project_location);
        output_dir.push(&self.output_directory);
        fs::create_dir_all(&output_dir).expect("Unable to create output directory");
        
        let mut output_file = output_dir.join(&self.name);
        match self.r#type {
            UnitType::Binary => {
                if cfg!(target_os = "windows") {
                    output_file.set_extension("exe");
                }
            },
            UnitType::StaticLibrary => {
                output_file.set_extension("lib");
            }
        }

        let project_compiler = project.compiler.clone().expect("Project compiler not selected");
        let compile_options = CompileOptions::new(
            self.name.clone(),
            self.r#type.clone(),
            source_file_paths,
            project.project_location.join(&self.output_directory),
            project.project_location.join(&self.intermediate_directory),
        );
        let compiler = compiler::get_compiler(project_compiler, compile_options);
        compiler.compile().unwrap();
        compiler.link().unwrap();
    }
}

impl CopperProject {
    /// Initialises the project with default values where possible
    pub fn init(name: String, directory: &Path) -> Self {
        Self {
            name,
            language: None,
            compiler: None,
            units: None,
            project_location: directory.to_path_buf()
        }
    }

    /// Imports a Copper project from a .toml project file
    pub fn import(directory: &Path) -> Self {
        let file_path = directory.join(PROJECT_FILE_NAME);
        let mut file = File::open(file_path).expect("File not found");

        let mut file_data = String::new();
        file.read_to_string(&mut file_data).expect("Unable to read the file");
        let mut project: CopperProject = toml::from_str(&file_data).expect("Unable to deserialize");
        project.project_location = directory.to_path_buf();
        project
    }

    /// Builds the whole project
    pub fn build(&self) {
        let units = &self.units;
        if let None = units {
            panic!("No units available to build");
        }

        for unit in units.clone().unwrap() {
            unit.build(self);
        }
    }

    /// Builds specified unit
    pub fn build_unit(&self, unit_name: &str) {
        todo!();
    }
}