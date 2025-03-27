use std::path::PathBuf;
use crate::compiler::{CompileOptions, Compiler, CompilerCommand, CompilerCommandFlags};
use crate::error::{Error, Result};
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
    source_files: Vec<PathBuf>,
    build_directory: PathBuf,
    intermediate_directory: PathBuf,
    include_paths: Option<Vec<PathBuf>>
}

impl From<CompileOptions> for GCCCompiler {
    /// Creates a new GCC Compiler from general compile options
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
            source_files: options.source_files,
            build_directory: options.output_directory,
            intermediate_directory: options.intermediate_directory,
            include_paths: options.include_paths
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
        for source_file in &self.source_files {
            let mut output_file = self.intermediate_directory.join(source_file.file_name().unwrap());
            output_file.set_extension("o");

            let mut command = self.command.executor();
            command.output(&output_file)?;
            command.compile(source_file, Some(&self.target_language.to_string()), &self.include_paths)?;
            let output = command.execute()?;

            if !output.status.success() {
                return Err(Error::CompileError(output));
            }
        }

        Ok(())
    }

    /// Links compiled object files to the output file
    fn link(&self) -> Result<()> {
        let mut object_files = self.source_files.iter()
            .map(|source_file| self.intermediate_directory.join(source_file.file_name().unwrap()))
            .collect::<Vec<PathBuf>>();
        object_files.iter_mut().for_each(|path| { path.set_extension("o"); });
        let output_file = self.build_directory.join(&self.target_name);

        let mut command = self.command.executor();
        command.output(&output_file)?;
        command.link(&object_files)?;
        let output = command.execute()?;

        if !output.status.success() {
            return Err(Error::LinkError(output));
        }
        
        Ok(())
    }
}