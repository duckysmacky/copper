use std::ffi::OsString;
use std::fmt::Display;
use std::path::PathBuf;
use std::{fs, io};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use serde::{Deserialize, Serialize};
use crate::compiler::TargetInformation;
use super::{ProjectConfig, Result, Error, ErrorKind};

/// A Copper unit configuration. This struct represents the contents of a unit entry in the
/// copper.yaml file
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct UnitConfig {
    /// Reference to the parent project configuration
    #[serde(skip)]
    project_reference: Weak<RefCell<ProjectConfig>>,
    /// Name of the unit
    pub name: String,
    /// Type of the unit
    pub r#type: UnitType,
    /// Location of the unit within the project (source code files)
    source: PathBuf,
    /// Unit's build output location.
    ///
    /// If there is no unit output directory specified, it will be generated from the project's
    /// defaults
    #[serde(default, skip_serializing_if = "Option::is_none")]
    output_directory: Option<PathBuf>,
    /// Unit's intermediate files location.
    ///
    /// If there is no unit output directory specified, it will be generated from the project's
    /// defaults
    #[serde(default, skip_serializing_if = "Option::is_none")]
    intermediate_directory: Option<PathBuf>,
    /// Pre-unit additional include paths
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    include_paths: Vec<PathBuf>,
    /// Per-unit additional compiler arguments
    #[serde(default, skip_serializing_if = "Option::is_none")]
    additional_compiler_args: Option<String>,
}

impl UnitConfig {
    pub fn new(
        project_reference: Weak<RefCell<ProjectConfig>>,
        name: String,
        r#type: UnitType,
        source: PathBuf,
    ) -> Self {
        UnitConfig {
            project_reference,
            name,
            r#type,
            source,
            output_directory: None,
            intermediate_directory: None,
            include_paths: Vec::new(),
            additional_compiler_args: None,
        }
    }
    
    /// Collects needed information about the unit and returns target information for later usage
    /// with a compiler
    pub fn get_target_information(&self) -> Result<TargetInformation> {
        if let Some(project) = self.get_project() {
            let project_config = project.borrow();
            // output and intermediate directories should be passed as relative to where the project is located
            let output_directory = self.get_output_directory(project_config.deref(), true);
            let intermediate_directory = self.get_intermediate_directory(project_config.deref(), true);

            let mut source_file_paths = Vec::new();
            let unit_path = project_config.root_path.join(&self.source);
            let extensions = project_config.language.extensions();
            if let Err(err) = self.get_source_files(&mut source_file_paths, unit_path, &extensions) {
                return Err(Error::new(ErrorKind::IOError("Unable to get unit's source files".to_string()), err));
            }

            if source_file_paths.is_empty() {
                return Err(Error::no_cause(ErrorKind::NoSourceFiles));
            }

            // TODO: move into compiler logic
            if let Err(err) = fs::create_dir_all(&output_directory) {
                if err.kind() != io::ErrorKind::AlreadyExists {
                    return Err(Error::new(ErrorKind::IOError("Unable to create unit's output directory".to_string()), err));
                }
            }

            Ok(TargetInformation::new(
                self.name.clone(),
                self.r#type.clone(),
                source_file_paths,
                output_directory,
                intermediate_directory,
                Some(self.include_paths.clone()),
                self.additional_compiler_args.clone(),
            ))
        } else {
            Err(Error::new(ErrorKind::ProjectUnavailable, "Unit's project reference is invalid"))
        }
    }
    
    /// Sets the parent project configuration reference for the unit
    pub fn set_project(&mut self, project: &Rc<RefCell<ProjectConfig>>) {
        self.project_reference = Rc::downgrade(project);
    }

    /// Returns a reference to the parent project configuration, if it is still valid (the project 
    /// itself was not dropped)
    pub fn get_project(&self) -> Option<Rc<RefCell<ProjectConfig>>> {
        self.project_reference.upgrade()
    }
    
    /// Returns the output directory for the unit, either specified in the unit itself or from the
    /// project defaults. If `rooted` is true, the path will be absolute (rooted at the project root),
    /// otherwise it will be relative to the project root
    fn get_output_directory(&self, project: &ProjectConfig, rooted: bool) -> PathBuf {
        let directory = self.output_directory.as_ref()
            .unwrap_or(match &self.r#type {
                UnitType::Binary => &project.defaults.binary_directory,
                _ => &project.defaults.library_directory,
            });
        
        if !rooted {
            return directory.clone();
        }
        
        project.root_path.join(directory)
    }
    
    /// Returns the intermediate directory for the unit, either specified in the unit itself or from the
    /// project defaults. If `rooted` is true, the path will be absolute (rooted at the project root),
    /// otherwise it will be relative to the project root
    fn get_intermediate_directory(&self, project: &ProjectConfig, rooted: bool) -> PathBuf {
        let directory = self.intermediate_directory.as_ref()
            .unwrap_or(&project.defaults.object_directory);
        
        if !rooted {
            return directory.clone();
        }
        
        project.root_path.join(directory)
    }

    /// Recursively searches the directory for the source files by extension (according to the
    /// language) and appends their paths to the vector of source file paths
    fn get_source_files(&self, source_paths: &mut Vec<PathBuf>, dir_path: PathBuf, extensions: &Vec<OsString>) -> io::Result<()> {
        for entry in fs::read_dir(&dir_path)? {
            let path = entry?.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if extensions.contains(&ext.to_os_string()) {
                        source_paths.push(path);
                    }
                }
            } else {
                self.get_source_files(source_paths, path, extensions)?;
            }
        }

        Ok(())
    }
}

/// Enum representing available project languages
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum UnitType {
    Binary,
    StaticLibrary,
    DynamicLibrary,
}

impl UnitType {
    const BINARY_STR: &'static str = "binary";
    const STATIC_LIBRARY_STR: &'static str = "static-library";
    const DYNAMIC_LIBRARY_STR: &'static str = "dynamic-library";

    /// Returns an array of possible unit type variants as stings
    pub fn str_variants() -> [&'static str; 3] {
        [Self::BINARY_STR, Self::STATIC_LIBRARY_STR, Self::DYNAMIC_LIBRARY_STR]
    }
}

impl Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            UnitType::Binary => Self::BINARY_STR,
            UnitType::StaticLibrary => Self::STATIC_LIBRARY_STR,
            UnitType::DynamicLibrary => Self::DYNAMIC_LIBRARY_STR,
        })
    }
}

impl TryFrom<String> for UnitType {
    type Error = Error;
    
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            Self::BINARY_STR => Ok(UnitType::Binary),
            Self::STATIC_LIBRARY_STR => Ok(UnitType::StaticLibrary),
            Self::DYNAMIC_LIBRARY_STR => Ok(UnitType::DynamicLibrary),
            _ => Err(Error::new(ErrorKind::InvalidUnitType, format!("'{}' is not a valid unit type", value))),
        }
    }
}

impl Into<String> for UnitType {
    fn into(self) -> String {
        match self {
            UnitType::Binary => Self::BINARY_STR.to_string(),
            UnitType::StaticLibrary => Self::STATIC_LIBRARY_STR.to_string(),
            UnitType::DynamicLibrary => Self::DYNAMIC_LIBRARY_STR.to_string(),
        }
    }
}