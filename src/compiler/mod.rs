use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use crate::error::{Error, Result};
use crate::project::{CopperProjectCompiler, UnitType};

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

/// Returns a specific Compiler based on the chosen project compiler
pub fn get_compiler(compiler: CopperProjectCompiler, options: CompileOptions) -> impl Compiler {
    match compiler {
        CopperProjectCompiler::GCC => gcc::GCCCompiler::from(options),
        _ => unimplemented!()
    }
}

/// Specifies the compiler-specific option flags
#[derive(Clone)]
struct CompilerCommandFlags {
    output: String,
    compile: String,
}

impl CompilerCommandFlags {
    fn new(output_flag: &str, compile_flag: &str) -> Self {
        CompilerCommandFlags {
            output: output_flag.to_string(),
            compile: compile_flag.to_string()
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
    executable: &'a str,
    option_flags: &'a CompilerCommandFlags,
    command: Command,
}

impl<'a> CompilerCommandExecutor<'a> {
    pub fn new(executable_name: &'a str, options_flags: &'a CompilerCommandFlags) -> Self {
        let command = Command::new(executable_name);

        CompilerCommandExecutor {
            executable: executable_name,
            option_flags: options_flags,
            command,
        }
    }

    /// Specify the expected output file
    pub fn output(&mut self, output_file: &Path) -> Result<()> {
        let output_dir = output_file.parent().unwrap();
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        self.command
            .arg(&self.option_flags.output)
            .arg(output_file);

        Ok(())
    }

    /// Compile provided source file into an object file
    pub fn compile(&mut self, source_file: &Path) -> Result<()> {
        if !source_file.exists() {
            return Err(Error::CompilerError(format!("File {:?} does not exist", source_file)))
        }

        self.command
            .arg(&self.option_flags.compile)
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
            .map_err(|err| Error::CompilerError(format!("Unable to spawn compiler process: {}", err)))?;

        if !output.status.success() {
            return Err(Error::CompilerError(self.handle_compiler_output(output)));
        }

        Ok(output)
    }

    fn handle_compiler_output(&self, output: Output) -> String {
        let mut message = String::new();
        message.push_str(format!("Compiler exited with code {}", output.status).as_str());

        if output.stdout.len() > 0 {
            message.push_str(format!("\nStdout:\n{}", String::from_utf8_lossy(&output.stdout)).as_str());
        }

        if output.stderr.len() > 0 {
            message.push_str(format!("\nStderr:\n{}", String::from_utf8_lossy(&output.stderr)).as_str());
        }

        message
    }
}