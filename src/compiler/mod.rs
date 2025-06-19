use std::process;
use std::path::PathBuf;
use crate::config::{ProjectCompiler, ProjectLanguage, UnitType};

mod gcc;
mod util;
mod command;

/// An instance of a specific compiler which is responsible for building, compiling and linking
/// project files
pub enum Compiler {
    GCC(gcc::Compiler),
}

impl Compiler {
    /// Returns a specific compiler instance based on the selected project compiler
    pub fn initialize(project_compiler: &ProjectCompiler, options: CompileOptions) -> Self {
        if !util::check_if_available(project_compiler) {
            eprintln!("Unsupported compiler specified (Not available on the current system)");
            process::exit(1);
        }

        match project_compiler {
            ProjectCompiler::GCC => Self::GCC(gcc::Compiler::from(options)),
            _ => unimplemented!()
        }
    }
    
    pub fn compile(&self) -> Result<(), String> { 
        match self {
           Self::GCC(compiler) => {
               if let Err(err) = compiler.compile() {
                   return Err(format!("GCC Error - {}", err))
               }
           }
        }
        
        Ok(())
    }

    pub fn link(&self) -> Result<(), String> {
        match self {
            Self::GCC(compiler) => {
                if let Err(err) = compiler.link() {
                    return Err(format!("GCC Error - {}", err))
                }
            }
        }
        
        Ok(())
    }
}

/// General compile options
pub struct CompileOptions {
    target_name: String,
    target_type: UnitType,
    target_language: ProjectLanguage,
    source_files: Vec<PathBuf>,
    output_directory: PathBuf,
    intermediate_directory: PathBuf,
    include_paths: Option<Vec<PathBuf>>
}

impl CompileOptions {
    pub fn new(
        target_name: String,
        target_type: UnitType,
        target_language: ProjectLanguage,
        target_source_files: Vec<PathBuf>,
        target_output_directory: PathBuf,
        target_intermediate_directory: PathBuf,
    ) -> Self {
        CompileOptions {
            target_name,
            target_type,
            target_language,
            source_files: target_source_files,
            output_directory: target_output_directory,
            intermediate_directory: target_intermediate_directory,
            include_paths: None,
        }
    }
    
    pub fn include_paths(&mut self, include_paths: Vec<PathBuf>) {
        self.include_paths = Some(include_paths);
    }
}