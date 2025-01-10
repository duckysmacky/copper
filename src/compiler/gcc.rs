use std::path::PathBuf;
use crate::compiler::{CompileOptions, Compiler, CompilerCommand, CompilerCommandFlags};
use crate::error::Result;
use crate::project::{CopperProjectLanguage, UnitType};

/// GCC-specific string constants
mod constants {
    pub const COMPILER_EXECUTABLE_NAME: &str = "gcc";

    pub mod flags {
        pub const OUTPUT: &str = "-o";
        pub const COMPILE: &str = "-c";
        pub const DIRECTORY: &str = "-I";
        pub const LANGUAGE: &str = "-x";
    }
}

/// Options for the GCC compiler
pub struct GCCCompiler {
    command: CompilerCommand,
    target_name: String,
    #[allow(dead_code)]
    target_type: UnitType,
    target_language: CopperProjectLanguage,
    target_source_files: Vec<PathBuf>,
    target_build_directory: PathBuf,
    target_intermediate_directory: PathBuf,
    target_include_paths: Option<Vec<PathBuf>>
}

impl From<CompileOptions> for GCCCompiler {
    /// Creates a new GCC Compiler from the compile options
    fn from(options: CompileOptions) -> Self {
        let compiler_flags = CompilerCommandFlags::new(
            constants::flags::OUTPUT,
            constants::flags::COMPILE,
            constants::flags::DIRECTORY,
            constants::flags::LANGUAGE
        );

        GCCCompiler {
            command: CompilerCommand::new(constants::COMPILER_EXECUTABLE_NAME, compiler_flags),
            target_name: options.target_name,
            target_type: options.target_type,
            target_language: options.target_language,
            target_source_files: options.target_source_files,
            target_build_directory: options.target_output_directory,
            target_intermediate_directory: options.target_intermediate_directory,
            target_include_paths: options.target_include_paths
        }
    }
}

impl Compiler for GCCCompiler {
    /// Build implementation for GCC
    fn build(&self) {
        if let Err(err) = self.compile() {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        
        if let Err(err) = self.link() {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

impl GCCCompiler {
    /// Compiles source files into object files
    fn compile(&self) -> Result<()> {
        for source_file in &self.target_source_files {
            let mut output_file = self.target_intermediate_directory.join(source_file.file_name().unwrap());
            output_file.set_extension("o");

            let mut command = self.command.executor();
            command.output(&output_file)?;
            command.compile(source_file, Some(&self.target_language.to_string()), &self.target_include_paths)?;
            command.execute()?;
        }

        Ok(())
    }

    /// Links compiled object files to the output file
    fn link(&self) -> Result<()> {
        let mut object_files = self.target_source_files.iter()
            .map(|source_file| self.target_intermediate_directory.join(source_file.file_name().unwrap()))
            .collect::<Vec<PathBuf>>();
        object_files.iter_mut().for_each(|path| { path.set_extension("o"); });
        let output_file = self.target_build_directory.join(&self.target_name);

        let mut command = self.command.executor();
        command.output(&output_file)?;
        command.link(&object_files)?;
        command.execute()?;
        
        Ok(())
    }
}