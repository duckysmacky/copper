use std::path::PathBuf;
use crate::error::Result;
use crate::project::{CopperProjectCompiler, UnitType};

mod gcc;

pub trait Compiler {
    fn compile(&self) -> Result<()>;
    fn link(&self) -> Result<()>;
}

/// General compile options
pub struct CompileOptions {
    target_name: String,
    target_type: UnitType,
    target_source_files: Vec<PathBuf>,
    target_output_directory: PathBuf,
    target_intermediate_directory: PathBuf,
}

impl CompileOptions {
    pub fn new(
        target_name: String,
        target_type: UnitType,
        target_source_files: Vec<PathBuf>,
        target_output_directory: PathBuf,
        target_intermediate_directory: PathBuf,
    ) -> Self {
        CompileOptions {
            target_name,
            target_type,
            target_source_files,
            target_output_directory,
            target_intermediate_directory,
        }
    }
}

// TODO
struct CompilerCommand {
    
}

pub fn get_compiler(compiler: CopperProjectCompiler, options: CompileOptions) -> impl Compiler {
    match compiler {
        CopperProjectCompiler::GCC => gcc::GCCCompiler::from(options),
        _ => unimplemented!()
    }
}