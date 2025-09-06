use std::process;
use std::fs::File;
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
use serde::{Deserialize, Serialize};
use crate::compiler::CompilerOptions;
use crate::project::default::ProjectDefaults;
use super::{ProjectLanguage, ProjectCompiler, UnitConfig, UnitType, PROJECT_FILE_NAME};

/// The main Copper project configuration. This struct represents the whole .yaml project configuration
/// file (copper.yaml)
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
    ) -> Self {
        ProjectConfig {
            root_path: project_location,
            name,
            language,
            compiler,
            global_include_paths: Vec::new(),
            global_additional_compiler_args: None,
            defaults: ProjectDefaults::default(),
            units: Vec::new(),
        }
    }
    
    /// Returns a vector of all unit names in the project
    pub fn get_unit_names(&self) -> Vec<&String> {
        self.units.iter().map(|u| &u.name).collect()
    }
    
    /// Returns a reference to a unit by its name, if it exists
    pub fn find_unit(&self, name: &str) -> Option<&UnitConfig> {
        self.units.iter().find(|u| u.name == name)
    }
}

/// Public Copper project interface for easier interaction
#[derive(Debug, Clone)]
pub struct CopperProject {
    config: Rc<RefCell<ProjectConfig>>,
}

impl CopperProject {
    pub fn new(
        project_location: PathBuf,
        name: String,
        language: ProjectLanguage,
        compiler: ProjectCompiler,
    ) -> Self {
        let config = ProjectConfig::new(project_location, name, language, compiler);
        Self { config: Rc::new(RefCell::new(config)) }
    }

    /// Imports a Copper project from a .yaml project file
    pub fn import(directory: &Path) -> io::Result<Self> {
        let file_path = directory.join(PROJECT_FILE_NAME);
        let mut file = File::open(file_path)?;

        let mut file_data = String::new();
        file.read_to_string(&mut file_data)?;

        let mut config: ProjectConfig = match serde_yaml::from_str(&file_data) {
            Ok(project) => project,
            Err(err) => {
                eprintln!("Unable to deserialize project");
                eprintln!("\tCause: {}", err);
                process::exit(1);
            }
        };

        config.root_path = directory.to_path_buf();
        let config = Rc::new(RefCell::new(config));
        
        // set correct project reference for each unit
        {
            let mut config_ref = config.borrow_mut();
            config_ref.units.iter_mut().for_each(|u| u.set_project(&config));
        }
        
        Ok(Self { config })
    }

    /// Saves current Copper project to the .yaml project file
    pub fn save(&self, directory: &Path) -> io::Result<()> {
        let config = self.get_config();
        let file_path = directory.join(PROJECT_FILE_NAME);
        let mut file = File::create(&file_path)?;

        let yaml_data = match serde_yaml::to_string(config.deref()) {
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

    /// Creates a new unit with minimum configuration and adds it to the project. This method
    /// **must** be used when creating new units programmatically as it ensures that the unit
    /// has a valid reference to the project configuration
    pub fn add_unit(&self, unit_name: String, unit_type: UnitType, unit_source: PathBuf) {
        let new_unit = UnitConfig::new(
            Rc::downgrade(&self.config),
            unit_name,
            unit_type,
            unit_source,
        );

        let mut config = self.get_config_mut();
        config.units.push(new_unit);
    }
    
    /// Returns global compiler options constructed from the project configuration
    pub fn get_compiler_options(&self) -> CompilerOptions {
        let config = self.get_config();
        
        CompilerOptions::new(
            config.root_path.clone(),
            config.language.clone(),
            Some(config.global_include_paths.clone()),
            config.global_additional_compiler_args.clone(),
        )
    }

    /// Access the underlying project config immutably
    pub fn get_config(&self) -> Ref<ProjectConfig> {
        self.config.borrow()
    }
    
    /// Access the underlying project config mutably
    pub fn get_config_mut(&self) -> RefMut<ProjectConfig> {
        self.config.borrow_mut()
    }
    
    /// Method which provides safe access to the underlying project config immutably. Executes the
    /// provided callback with a reference to the project config
    pub fn with_config<F, T>(&self, callback: F) -> T
    where
        F: FnOnce(&ProjectConfig) -> T,
    {
        let config_ref = self.get_config();
        callback(&config_ref)
    }
    
    /// Method which provides safe access to the underlying project config mutably. Executes the
    /// provided callback with a mutable reference to the project config
    pub fn with_config_mut<F, T>(&self, callback: F) -> T
    where
        F: FnOnce(&mut ProjectConfig) -> T,
    {
        let mut config_ref = self.get_config_mut();
        callback(&mut config_ref)
    }
}