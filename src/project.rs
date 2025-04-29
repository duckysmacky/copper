use std::ffi::OsString;
use std::fmt::Display;
use std::string::String;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::vec;
use serde::{Deserialize, Serialize};
use crate::compiler::{self, CompileOptions, Compiler};
use crate::error::{Result, Error};

mod default {
    use std::path::PathBuf;
    type Val<T> = fn() -> T;

    pub const BUILD_DIRECTORY: Val<PathBuf> = || PathBuf::from("build/");
}

pub const PROJECT_FILE_NAME: &str = "copper.toml";
#[allow(dead_code)]
pub const PROJECT_DIRECTORY_NAME: &str = ".copper";

/// Main Copper project configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct CopperProject {
    /// Name of the project
    name: String,
    /// Chosen language for the project
    language: CopperProjectLanguage,
    /// Chosen compiler for the project
    compiler: CopperProjectCompiler,
    /// Additional include paths for the whole project
    include_paths: Option<Vec<PathBuf>>,
    /// Unit configuration data
    #[serde(rename = "Unit")]
    units: Vec<CopperUnit>,
    /// Default build directory for all new units
    #[serde(default = "default::BUILD_DIRECTORY")]
    #[serde(skip_serializing)]
    default_build_directory: PathBuf,
    /// Location of the Copper project relative to where the command was executed.
    #[serde(skip)]
    project_location: PathBuf
}

impl CopperProject {
    /// Initialises the project with default values where possible and generates a new .toml file
    pub fn init(
        project_location: &Path,
        project_name: String,
        project_language: CopperProjectLanguage,
        generate_example: bool
    ) -> Result<PathBuf> {
        let default_compiler = if cfg!(target_os = "windows") {
            CopperProjectCompiler::MSVC
        } else {
            match &project_language {
                CopperProjectLanguage::C => CopperProjectCompiler::GCC,
                CopperProjectLanguage::CPP => CopperProjectCompiler::GPP
            }
        };

        let mut include_paths = None;
        let mut units = Vec::new();

        if generate_example {
            let src_dir = project_location.join("src");
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

/// Configuration for the project unit
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

impl CopperUnit {
    pub fn new(
        name: String,
        r#type: UnitType,
        source: PathBuf,
        output_directory: PathBuf,
        intermediate_directory: PathBuf
    ) -> Self {
        CopperUnit {
            name,
            r#type,
            source,
            output_directory,
            intermediate_directory
        }
    }

    /// Collects needed information about the unit and builds it according to its type and selected
    /// project compiler
    pub fn build(&self, project: &CopperProject) -> Result<()> {
        let unit_path = project.project_location.join(&self.source);
        let unit_dir = fs::read_dir(&unit_path)?;

        let mut source_file_paths: Vec<PathBuf> = Vec::new();
        for entry in unit_dir {
            let file_path = entry?.path();

            if let Some(ext) = file_path.extension() {
                if project.language.extensions().contains(&ext.to_os_string()) {
                    source_file_paths.push(file_path);
                }
            }
        }

        let mut output_dir = PathBuf::from(&project.project_location);
        output_dir.push(&self.output_directory);
        fs::create_dir_all(&output_dir)?;

        let mut output_file = output_dir.join(&self.name);
        match self.r#type {
            UnitType::Binary => {
                if cfg!(target_os = "windows") {
                    output_file.set_extension("exe");
                }
            },
            UnitType::StaticLibrary => todo!()
        }

        let mut compile_options = CompileOptions::new(
            self.name.clone(),
            self.r#type.clone(),
            project.language.clone(),
            source_file_paths,
            project.project_location.join(&self.output_directory),
            project.project_location.join(&self.intermediate_directory),
        );

        if let Some(include_paths) = &project.include_paths {
            let include_paths: Vec<PathBuf> = include_paths.iter()
                .map(|path| project.project_location.join(path))
                .collect();
            compile_options.include_paths(include_paths);
        }
        
        let compiler = compiler::get_compiler(&project.compiler, compile_options);
        compiler.build();

        Ok(())
    }
}

/// Enum representing available project languages
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum CopperProjectLanguage {
    C,
    CPP
}

impl CopperProjectLanguage {
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
            CopperProjectLanguage::C => Vec::from(Self::C_EXTENSIONS),
            CopperProjectLanguage::CPP => Vec::from(Self::CPP_EXTENSIONS)
        }.into_iter()
            .map(OsString::from)
            .collect()
    }
}

impl TryFrom<String> for CopperProjectLanguage {
    type Error = Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            Self::C_STR => Ok(CopperProjectLanguage::C),
            Self::CPP_STR => Ok(CopperProjectLanguage::CPP),
            _ => Err(Error::EnumParseError(format!("Unexpected language value: {}", value)))
        }
    }
}

impl Into<String> for CopperProjectLanguage {
    fn into(self) -> String {
        match self {
            CopperProjectLanguage::C => Self::C_STR.to_string(),
            CopperProjectLanguage::CPP => Self::CPP_STR.to_string()
        }
    }
}

impl Display for CopperProjectLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CopperProjectLanguage::C => Self::C_STR.to_string(),
            CopperProjectLanguage::CPP => Self::CPP_STR.to_string()
        };
        write!(f, "{}", str)
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
    
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
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

/// Enum representing available project languages
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum UnitType {
    Binary,
    StaticLibrary
}

impl UnitType {
    const BINARY_STR: &'static str = "binary";
    const STATIC_LIBRARY_STR: &'static str = "static-library";

    /// Returns an array of possible unit type variants as stings
    pub fn get_strings() -> [&'static str; 2] {
        [
            Self::BINARY_STR,
            Self::STATIC_LIBRARY_STR
        ]
    }
}

impl Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UnitType::Binary => UnitType::BINARY_STR.to_string(),
            UnitType::StaticLibrary => UnitType::STATIC_LIBRARY_STR.to_string()
        };
        write!(f, "{}", str)
    }
}

impl TryFrom<String> for UnitType {
    type Error = Error;
    
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            UnitType::BINARY_STR => Ok(UnitType::Binary),
            UnitType::STATIC_LIBRARY_STR => Ok(UnitType::StaticLibrary),
            _ => Err(Error::EnumParseError(format!("Unexpected unit type value: {}", value)))
        }
    }
}

impl Into<String> for UnitType {
    fn into(self) -> String {
        match self {
            UnitType::Binary => UnitType::BINARY_STR.to_string(),
            UnitType::StaticLibrary => UnitType::STATIC_LIBRARY_STR.to_string()
        }
    }
}