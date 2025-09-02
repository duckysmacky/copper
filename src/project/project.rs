use std::process;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::compiler::CompilerOptions;
use crate::project::default::ProjectDefaults;
use super::{ProjectLanguage, ProjectCompiler, UnitConfig, UnitType, PROJECT_FILE_NAME};

/// Main Copper project configuration file
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectConfig {
    /// Location of the Copper project relative to where the command was executed.
    #[serde(skip)]
    pub project_location: PathBuf,
    /// Name of the project
    pub name: String,
    /// Chosen language for the project
    pub language: ProjectLanguage,
    /// Chosen compiler for the project
    pub compiler: ProjectCompiler,
    /// Project-wide additional include paths
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_include_paths: Option<Vec<PathBuf>>,
    /// Project-wide additional compiler arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_additional_compiler_args: Option<String>,
    /// Project-specific default values
    #[serde(default = "ProjectDefaults::default")]
    #[serde(skip_serializing_if = "ProjectDefaults::all_default")]
    pub defaults: ProjectDefaults,
    /// Unit configuration data
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
            global_include_paths,
            global_additional_compiler_args: global_compiler_args,
            defaults: ProjectDefaults::default(),
            units,
        }
    }

    /// Imports a Copper project from a .yaml project file
    pub fn import(directory: &Path) -> io::Result<Self> {
        let file_path = directory.join(PROJECT_FILE_NAME);
        let mut file = File::open(file_path)?;

        let mut file_data = String::new();
        file.read_to_string(&mut file_data)?;

        let mut project: ProjectConfig = match serde_yaml::from_str(&file_data) {
            Ok(project) => project,
            Err(err) => {
                eprintln!("Unable to deserialize project: {}", err);
                process::exit(1);
            }
        };

        project.project_location = directory.to_path_buf();
        Ok(project)
    }

    /// Saves current Copper project to the .yaml project file
    pub fn save(self, directory: &Path) -> io::Result<()> {
        let file_path = directory.join(PROJECT_FILE_NAME);
        let mut file = File::create(&file_path)?;


        let yaml_data = match serde_yaml::to_string(&self) {
            Ok(yaml) => yaml,
            Err(err) => {
                eprintln!("Unable to serialize project: {}", err);
                process::exit(1);
            }
        };

        file.write_all(yaml_data.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    /// Creates a new unit with minimum configuration and adds it to the project
    pub fn add_unit(&mut self, unit_name: String, unit_type: UnitType, unit_source: PathBuf) {
        self.units.push(UnitConfig::new(
            unit_name,
            unit_type,
            unit_source,
            None,
            None,
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