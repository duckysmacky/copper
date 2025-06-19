use std::process;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::compiler::CompilerOptions;
use super::{default, equals, ProjectLanguage, ProjectCompiler, UnitConfig, UnitType, PROJECT_FILE_NAME};

/// Main Copper project configuration file
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectConfig {
    /// Location of the Copper project relative to where the command was executed.
    #[serde(skip)]
    pub project_location: PathBuf,
    /// Name of the project
    name: String,
    /// Chosen language for the project
    pub language: ProjectLanguage,
    /// Chosen compiler for the project
    pub compiler: ProjectCompiler,
    /// Default build directory path for all units
    #[serde(default = "default::BUILD_DIRECTORY")]
    #[serde(skip_serializing_if = "equals::BUILD_DIRECTORY")]
    pub default_build_directory: PathBuf,
    /// Default binary directory path for all units
    #[serde(default = "default::BINARY_DIRECTORY")]
    #[serde(skip_serializing_if = "equals::BINARY_DIRECTORY")]
    pub default_binary_directory: PathBuf,
    /// Default library directory path for all units
    #[serde(default = "default::LIBRARY_DIRECTORY")]
    #[serde(skip_serializing_if = "equals::LIBRARY_DIRECTORY")]
    pub default_library_directory: PathBuf,
    /// Default object files directory path for all units
    #[serde(default = "default::OBJECT_DIRECTORY")]
    #[serde(skip_serializing_if = "equals::OBJECT_DIRECTORY")]
    pub default_object_directory: PathBuf,
    /// Project-wide additional include paths
    pub global_include_paths: Option<Vec<PathBuf>>,
    /// Project-wide additional compiler arguments
    pub global_additional_compiler_args: Option<String>,
    /// Unit configuration data
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    units: Vec<UnitConfig>,
}

impl ProjectConfig {
    pub fn new(
        project_location: PathBuf,
        name: String,
        language: ProjectLanguage,
        compiler: ProjectCompiler,
        global_include_paths: Option<Vec<PathBuf>>,
        global_compiler_args: Option<String>,
        units: Vec<UnitConfig>,
    ) -> Self {
        ProjectConfig {
            project_location,
            name,
            language,
            compiler,
            default_build_directory: default::BUILD_DIRECTORY(),
            default_binary_directory: default::BINARY_DIRECTORY(),
            default_library_directory: default::LIBRARY_DIRECTORY(),
            default_object_directory: default::OBJECT_DIRECTORY(),
            global_include_paths,
            global_additional_compiler_args: global_compiler_args,
            units,
        }
    }

    /// Imports a Copper project from a .toml project file
    pub fn import(directory: &Path) -> io::Result<Self> {
        let file_path = directory.join(PROJECT_FILE_NAME);
        let mut file = File::open(file_path)?;

        let mut file_data = String::new();
        file.read_to_string(&mut file_data)?;

        let mut project: ProjectConfig = match toml::from_str(&file_data) {
            Ok(project) => project,
            Err(err) => {
                eprintln!("Unable to deserialize project: {}", err);
                process::exit(1);
            }
        };

        project.project_location = directory.to_path_buf();
        Ok(project)
    }

    /// Saves current Copper project to the .toml project file
    pub fn save(self, directory: &Path) -> io::Result<()> {
        let file_path = directory.join(PROJECT_FILE_NAME);
        let mut file = File::create(&file_path)?;

        let toml_data = match toml::to_string(&self) {
            Ok(toml) => toml,
            Err(err) => {
                eprintln!("Unable to serialize project: {}", err);
                process::exit(1);
            }
        };

        file.write_all(toml_data.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    /// Creates a new unit with minimum configuration and adds it to the project
    pub fn add_unit(&mut self, unit_name: String, unit_type: UnitType, unit_source: PathBuf) {
        let unit_type_directory = match &unit_type {
            UnitType::Binary => &self.default_binary_directory,
            UnitType::StaticLibrary | UnitType::DynamicLibrary => &self.default_library_directory,
        };

        self.units.push(UnitConfig::new(
            unit_name,
            unit_type,
            unit_source,
            self.default_build_directory.join(unit_type_directory),
            self.default_build_directory.join(&self.default_object_directory),
            None,
            None,
        ))
    }

    /// Searches for a unit in project by the provided name. If not found, returns None
    pub fn find_unit(&self, unit_name: &str) -> Option<&UnitConfig> {
        let unit = self.units.iter()
            .find(|u| &u.name == unit_name);

        if let Some(unit) = unit {
            Some(&unit)
        } else {
           None
        }
    }

    /// Returns an iterator containing the names of the all project units
    pub fn get_unit_names(&self) -> Vec<&String> {
        self.units.iter()
        .map(|unit| &unit.name)
        .collect()
    }
    
    pub fn get_compiler_options(&self) -> CompilerOptions {
        CompilerOptions::new(
            self.project_location.clone(),
            self.language.clone(),
            self.global_include_paths.clone(),
            self.global_additional_compiler_args.clone(),
        )
    }
}