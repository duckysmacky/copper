use std::process;
use std::path::PathBuf;
use crate::config::{ProjectCompiler, ProjectLanguage, UnitType};

mod gcc;
mod util;
mod command;

/// Compiler trait to generically refer to
pub trait Compiler {
    /// Builds the unit with the provided Compile Options using the selected compiler
    fn compile(&self) -> Result<(), impl CompilerError>;
    fn link(&self) -> Result<(), impl CompilerError>;
}

/// A trait representing that the error is a compiler-specific error
pub trait CompilerError {
    /// Display the error in a pretty way
    fn display(&self) -> String;
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

/// Returns a specific Compiler based on the chosen project compiler
pub fn get_compiler(compiler: &ProjectCompiler, options: CompileOptions) -> impl Compiler {
    if !util::check_if_available(compiler) {
        eprintln!("Unsupported compiler specified (Not available on the current system)");
        process::exit(1);
    }
    
    match compiler {
        ProjectCompiler::GCC => gcc::GCCCompiler::from(options),
        _ => unimplemented!()
    }
}
