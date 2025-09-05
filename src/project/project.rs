use std::process;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
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
    pub root_path: PathBuf,
    /// Name of the project
    pub name: String,
    /// Chosen language for the project
    pub language: ProjectLanguage,
    /// Chosen compiler for the project
    pub compiler: ProjectCompiler,
    /// Project-wide additional include paths
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub global_include_paths: Vec<PathBuf>,
    /// Project-wide additional compiler arguments
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_additional_compiler_args: Option<String>,
    /// Project-specific default values
    #[serde(default, skip_serializing_if = "ProjectDefaults::all_default")]
    pub defaults: ProjectDefaults,
    /// Unit configuration data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    units: Vec<UnitConfig>,
}

impl ProjectConfig {
    pub fn new(
        project_location: PathBuf,
        name: String,
        language: ProjectLanguage,
        compiler: ProjectCompiler,
    ) -> Rc<Self> {
        Rc::new(Self {
            root_path: project_location,
            name,
            language,
            compiler,
            global_include_paths: Vec::new(),
            global_additional_compiler_args: None,
            defaults: ProjectDefaults::default(),
            units: Vec::new(),
        })
    }

    /// Imports a Copper project from a .yaml project file
    pub fn import(directory: &Path) -> io::Result<Rc<Self>> {
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

        project.root_path = directory.to_path_buf();
        Ok(Rc::new(project))
    }

    /// Saves current Copper project to the .yaml project file
    pub fn save(&self, directory: &Path) -> io::Result<()> {
        let file_path = directory.join(PROJECT_FILE_NAME);
        let mut file = File::create(&file_path)?;

        let yaml_data = match serde_yaml::to_string(self) {
            Ok(yaml) => yaml,
            Err(err) => {
                eprintln!("Unable to serialize project");
                eprintln!("\tCause: {}", err);
                process::exit(1);
            }
        };

        file.write_all(yaml_data.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    /// Creates a new unit with minimum configuration and adds it to the project
    pub fn add_unit(self: &mut Rc<Self>, unit_name: String, unit_type: UnitType, unit_source: PathBuf) {
        let new_unit = UnitConfig::new(
            Rc::downgrade(self),
            unit_name,
            unit_type,
            unit_source,
        );
        
        match Rc::get_mut(self) {
            Some(project) => project.units.push(new_unit),
            None => {
                eprintln!("Unable to add a new unit to the project");
                eprintln!("\tCause: multiple mutable references to the project already exist");
                process::exit(1);
            }
        }
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
            self.root_path.clone(),
            self.language.clone(),
            Some(self.global_include_paths.clone()),
            self.global_additional_compiler_args.clone(),
        )
    }
}