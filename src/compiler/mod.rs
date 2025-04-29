use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use crate::error::{Error, Result};
use crate::config::project::{ProjectCompiler, ProjectLanguage};
use crate::config::unit::UnitType;

mod gcc;

/// Compiler trait to generically refer to
pub trait Compiler {
    /// Builds the unit with the provided Compile Options using the selected compiler
    fn build(&self);
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
    match compiler {
        ProjectCompiler::GCC => gcc::GCCCompiler::from(options),
        _ => unimplemented!()
    }
}

/// Specifies the compiler-specific option flags
#[derive(Clone)]
struct CompilerCommandFlags {
    output: String,
    compile: String,
    include_directory: String,
    language: String
}

impl CompilerCommandFlags {
    fn new(
        output_flag: &str,
        compile_flag: &str,
        include_directory: &str,
        language: &str,
    ) -> Self {
        CompilerCommandFlags {
            output: output_flag.to_string(),
            compile: compile_flag.to_string(),
            include_directory: include_directory.to_string(),
            language: language.to_string()
        }
    }
}

/// Wrapper for the compiler command executor
struct CompilerCommand {
    executable_name: String,
    compiler_flags: CompilerCommandFlags,
}

impl CompilerCommand {
    pub fn new(executable_name: &str, options_flags: CompilerCommandFlags) -> Self {
        CompilerCommand {
            executable_name: executable_name.to_string(),
            compiler_flags: options_flags,
        }
    }

    /// Initiates a new Executor to use
    pub fn executor<'a>(&self) -> CompilerCommandExecutor {
        CompilerCommandExecutor::new(
            &self.executable_name,
            &self.compiler_flags
        )
    }
}

/// Executor for the compiler command itself
struct CompilerCommandExecutor<'a> {
    command: Command,
    flags: &'a CompilerCommandFlags,
}

impl<'a> CompilerCommandExecutor<'a> {
    pub fn new(executable_name: &str, flags: &'a CompilerCommandFlags) -> Self {
        let command = Command::new(executable_name);

        CompilerCommandExecutor {
            command,
            flags,
        }
    }

    /// Specify the expected output file
    pub fn output(&mut self, output_file: &Path) -> Result<()> {
        let output_dir = output_file.parent().unwrap();
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        self.command
            .arg(&self.flags.output)
            .arg(output_file);

        Ok(())
    }

    /// Compile provided source file into an object file
    pub fn compile(&mut self, source_file: &Path, language: Option<&str>, include_paths: &Option<Vec<PathBuf>>) -> Result<()> {
        if !source_file.exists() {
            return Err(Error::CompilerError(format!("File {:?} does not exist", source_file)))
        }

        if let Some(language) = language {
            self.command
                .arg(&self.flags.language)
                .arg(language);
        }

        if let Some(include_paths) = include_paths {
            for include_path in include_paths {
                if !include_path.exists() {
                    return Err(Error::CompilerError(format!("Directory {:?} does not exist", include_path)))
                }

                self.command
                    .arg(&self.flags.include_directory)
                    .arg(include_path);
            }
        }

        self.command
            .arg(&self.flags.compile)
            .arg(source_file);

        Ok(())
    }

    /// Link object files into a single file
    pub fn link(&mut self, object_files: &Vec<PathBuf>) -> Result<()> {
        for object_file in object_files {
            if !object_file.exists() {
                return Err(Error::CompilerError(format!("File {:?} does not exist", object_file)))
            }
        }

        self.command
            .args(object_files);

        Ok(())
    }

    /// Consumes itself and spawns the process, waits for its completion and returns the output
    pub fn execute(mut self) -> Result<Output> {
        let output = self.command.output()
            .map_err(|err| Error::CompilerError(format!("Unable to spawn compiler process ({})", err)))?;

        Ok(output)
    }
}