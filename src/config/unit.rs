use std::ffi::OsString;
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io, process};
use crate::compiler::CompileOptions;
use crate::config::project::CopperProject;
use super::{default, equals, Error, Result};

/// Configuration for the project unit
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CopperUnit {
    /// Name of the unit
    pub name: String,
    /// Type of the unit
    pub r#type: UnitType,
    /// Location of the unit within the project (source code files)
    source: PathBuf,
    /// Per-unit build output location
    #[serde(default = "default::LOCAL_DIRECTORY_PATH")]
    #[serde(skip_serializing_if = "equals::LOCAL_DIRECTORY_PATH")]
    output_directory: PathBuf,
    /// Per-unit location for intermediate files
    #[serde(default = "default::LOCAL_DIRECTORY_PATH")]
    #[serde(skip_serializing_if = "equals::LOCAL_DIRECTORY_PATH")]
    intermediate_directory: PathBuf,
    /// Pre-unit additional include paths
    include_paths: Option<Vec<PathBuf>>,
    /// Per-unit additional compiler arguments
    additional_compiler_args: Option<String>,
}

impl CopperUnit {
    pub fn new(
        name: String,
        r#type: UnitType,
        source: PathBuf,
        output_directory: PathBuf,
        intermediate_directory: PathBuf,
        include_paths: Option<Vec<PathBuf>>,
        additional_compiler_args: Option<String>,
    ) -> Self {
        CopperUnit {
            name,
            r#type,
            source,
            output_directory,
            intermediate_directory,
            include_paths,
            additional_compiler_args,
        }
    }

    /// Collects needed information about the unit and builds it according to its type and selected
    /// project compiler and returns compile options for later usage with a compiler
    pub fn get_compile_options(&self, parent_project: &CopperProject) -> Result<CompileOptions> {
        let unit_path = parent_project.project_location.join(&self.source);

        let mut source_file_paths = Vec::new();
        if let Err(err) =  self.get_source_files(&mut source_file_paths, unit_path, &parent_project.language.extensions()) {
            eprintln!("Unable to get unit's source files: {}", err.to_string());
            process::exit(1);
        }

        if source_file_paths.is_empty() {
            return Err(Error::NoSourceFiles);
        }

        let output_dir = parent_project.project_location.join(&self.output_directory);
        if let Err(err) = fs::create_dir_all(&output_dir) {
            if err.kind() != io::ErrorKind::AlreadyExists {
                eprintln!("Unable to create unit's output directory: {}", err.to_string());
                process::exit(1);
            }
        }

        let mut output_file = output_dir.join(&self.name);
        match self.r#type {
            UnitType::Binary => {
                if cfg!(windows) {
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

        let mut include_paths = Vec::new();

        // Global
        if let Some(paths) = &parent_project.global_include_paths {
            paths.iter()
                .map(|path| parent_project.project_location.join(path))
                .for_each(|path| include_paths.push(path));
        }

        // Unit
        if let Some(paths) = &self.include_paths {
            paths.iter()
                .map(|path| parent_project.project_location.join(path))
                .for_each(|path| include_paths.push(path));
        }

        if !include_paths.is_empty() {
            compile_options.include_paths(include_paths);
        }
        
        Ok(compile_options)
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
            _ => Err(Error::InvalidUnitType(value)),
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