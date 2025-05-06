use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use crate::compiler::{CompileOptions, Compiler, CompilerCommand, CompilerCommandFlags, CompilerError};
use crate::config::project::ProjectLanguage;
use crate::config::unit::UnitType;
use crate::error::parse_output;

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

/// GCC-specific error types
enum Error {
    /// Error related to the compilation of the source files
    CompileError(String),
    /// Error related to the linking of the object files
    LinkError(String),
}

impl CompilerError for Error {
    fn display(&self) -> String {
        format!("GCC Error - {}", self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CompileError(s) => write!(f, "Compiling failed ({})", s),
            Error::LinkError(s) => write!(f, "Linking failed ({})", s),
        }   
    }
}

/// Options for the GCC compiler
pub struct GCCCompiler {
    command: CompilerCommand,
    target_name: String,
    #[allow(dead_code)]
    target_type: UnitType,
    target_language: ProjectLanguage,
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
    /// Compiles source files into object files
    fn compile(&self) -> std::result::Result<(), impl CompilerError> {
        for source_file in &self.source_files {
            let mut output_file = self.intermediate_directory.join(source_file.file_name().unwrap());
            output_file.set_extension("o");

            let mut command = self.command.executor();
            command.output(&output_file)
                .map_err(|err| Error::CompileError(err.to_string()))?;
            command.compile(source_file, Some(&self.target_language.to_string()), &self.include_paths)
                .map_err(|err| Error::CompileError(err.to_string()))?;
            let output = command.execute()
                .map_err(|err| Error::CompileError(err.to_string()))?;

            if !output.status.success() {
                return Err(Error::CompileError(parse_output(&output)));
            }
        }

        Ok(())
    }

    /// Links compiled object files to the output file
    fn link(&self) -> std::result::Result<(), impl CompilerError> {
        let mut object_files = self.source_files.iter()
            .map(|source_file| self.intermediate_directory.join(source_file.file_name().unwrap()))
            .collect::<Vec<PathBuf>>();
        object_files.iter_mut().for_each(|path| { path.set_extension("o"); });
        let output_file = self.build_directory.join(&self.target_name);

        let mut command = self.command.executor();
        command.output(&output_file)
            .map_err(|err| Error::LinkError(err.to_string()))?;
        command.link(&object_files)
            .map_err(|err| Error::LinkError(err.to_string()))?;
        let output = command.execute()
            .map_err(|err| Error::LinkError(err.to_string()))?;

        if !output.status.success() {
            return Err(Error::LinkError(parse_output(&output)));
        }
        
        Ok(())
    }
}