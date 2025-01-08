use std::path::PathBuf;
use std::process::{Child, Command, Output};
use crate::compiler::{CompileOptions, Compiler};
use crate::error::{Result, Error};
use crate::project::UnitType;

const COMPILER_EXECUTABLE: &str = "gcc";

/// GCC-specific compiler options
pub struct GCCCompiler {
    target_name: String,
    target_type: UnitType,
    target_source_files: Vec<PathBuf>,
    target_build_directory: PathBuf,
    target_intermediate_directory: PathBuf,
}

impl From<CompileOptions> for GCCCompiler {
    fn from(options: CompileOptions) -> Self {
        GCCCompiler {
            target_name: options.target_name,
            target_type: options.target_type,
            target_source_files: options.target_source_files,
            target_build_directory: options.target_output_directory,
            target_intermediate_directory: options.target_intermediate_directory
        }
    }
}

impl Compiler for GCCCompiler {
    fn compile(&self) -> Result<()> {
        for source_file in &self.target_source_files {
            let out = self.compile_object(source_file)?;
            println!("Compilation of {:?} completed successfully", source_file);
        }
        
        Ok(())
    }

    fn link(&self) -> Result<()> {
        let mut object_files = self.target_source_files.iter()
            .map(|source_file| self.target_intermediate_directory.join(source_file.file_name().unwrap()))
            .collect::<Vec<PathBuf>>();
        object_files.iter_mut().for_each(|path| { path.set_extension("o"); });
        
        self.link_binary(&object_files)?;
        println!("Linking of {:?} completed successfully", object_files);
        
        Ok(())
    }
}

// TODO: make wait for previous command to finish
impl GCCCompiler {
    fn compile_object(&self, source_file: &PathBuf) -> Result<Child> {
        let mut output_file = self.target_intermediate_directory.join(source_file.file_name().unwrap());
        output_file.set_extension("o");
        Command::new(COMPILER_EXECUTABLE)
            .arg("-o").arg(&output_file)
            .arg("-c").arg(&source_file)
            .spawn()
            .map_err(|err| Error::CompilerError(err.to_string()))
    }

    fn link_binary(&self, object_files: &Vec<PathBuf>) -> Result<Child> {
        let output_file = self.target_build_directory.join(&self.target_name);
        Command::new(COMPILER_EXECUTABLE)
            .arg("-o").arg(&output_file)
            .args(object_files)
            .spawn()
            .map_err(|err| Error::CompilerError(err.to_string()))
    }
}