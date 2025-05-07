use std::fmt::Display;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use crate::compiler::CompileOptions;
use crate::config::project::CopperProject;
use crate::error::{Error, Result};

/// Configuration for the project unit
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CopperUnit {
    /// Name of the unit
    pub name: String,
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
    /// project compiler and returns compile options for later usage with a compiler
    pub fn get_compile_options(&self, parent_project: &CopperProject) -> Result<CompileOptions> {
        let unit_path = parent_project.project_location.join(&self.source);
        let unit_dir = fs::read_dir(&unit_path)?;

        let mut source_file_paths: Vec<PathBuf> = Vec::new();
        for entry in unit_dir {
            let file_path = entry?.path();

            if let Some(ext) = file_path.extension() {
                if parent_project.language.extensions().contains(&ext.to_os_string()) {
                    source_file_paths.push(file_path);
                }
            }
        }

        let mut output_dir = PathBuf::from(&parent_project.project_location);
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
            parent_project.language.clone(),
            source_file_paths,
            parent_project.project_location.join(&self.output_directory),
            parent_project.project_location.join(&self.intermediate_directory),
        );

        if let Some(include_paths) = &parent_project.include_paths {
            let include_paths: Vec<PathBuf> = include_paths.iter()
                .map(|path| parent_project.project_location.join(path))
                .collect();
            compile_options.include_paths(include_paths);
        }
        
        Ok(compile_options)
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