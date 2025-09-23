use std::process::{Command, Output};
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::borrow::Cow;
use crate::project::ProjectLanguage;

/// Specifies the compiler-specific option flags
pub struct CompilerCommandFlags {
    pub output: &'static str,
    pub compile: &'static str,
    pub include: &'static str,
    pub language: &'static str,
}

/// Wrapper for the compiler command executor
pub struct CompilerCommand {
    /// Name of the compiler executable to use
    compiler_executable: String,
    /// Compiler-specific flags
    compiler_flags: CompilerCommandFlags,
    /// Relative path to the project root from the process's location to correctly supply
    /// path-based compiler arguments
    root_relative_path: PathBuf,
}

impl CompilerCommand {
    pub fn new(
        executable_name: String,
        command_flags: CompilerCommandFlags,
        root_relative_path: PathBuf,
    ) -> Self {
        CompilerCommand {
            compiler_executable: executable_name,
            compiler_flags: command_flags,
            root_relative_path,
        }
    }

    /// Initiates a instance of an Executor to use
    pub fn executor<'a>(&self) -> CompilerCommandExecutor {
        CompilerCommandExecutor::new(
            &self.compiler_executable,
            &self.compiler_flags,
            &self.root_relative_path,
        )
    }
}

/// Executor for the compiler command itself
pub struct CompilerCommandExecutor<'a> {
    command: Command,
    flags: &'a CompilerCommandFlags,
    relative_path: &'a Path,
}

impl<'a> CompilerCommandExecutor<'a> {
    pub fn new(
        executable_name: &str, 
        flags: &'a CompilerCommandFlags,
        relative_path: &'a Path,
    ) -> Self {
        CompilerCommandExecutor {
            command: Command::new(executable_name),
            flags,
            relative_path,
        }
    }
    
    /// Add the compile flag to the compiler command
    pub fn set_compile_flag(&mut self) {
        self.command
            .arg(&self.flags.compile);
    }
    
    /// Specify the language for the compiler
    pub fn set_language(&mut self, language: &ProjectLanguage) {
        self.command
            .arg(&self.flags.language)
            .arg(language.to_string());
    }

    /// Specify the output file and make sure all the parent directories exist
    pub fn set_output_file(&mut self, output_file: &Path) -> io::Result<()> {
        if let Some(output_dir) = output_file.parent() {
            if output_dir.is_dir() && !output_dir.exists() {
                fs::create_dir_all(output_dir)?;
            }
        }

        self.command
            .arg(&self.flags.output)
            .arg(output_file);

        Ok(())
    }
    
    /// Add an include path to the compiler command and verify that it exists
    pub fn add_include_path(&mut self, include_path: &Path) -> io::Result<()> {
        let include_path = self.relative_path.join(include_path);
        
        if !include_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound, 
                format!("Include path '{}' does not exist", include_path.display())
            ));
        }
        
        self.command
            .arg(&self.flags.include)
            .arg(include_path);
        
        Ok(())
    }

    /// Add an input file to the compiler command and verify that it exists
    pub fn add_input_file(&mut self, source_file: &Path) -> io::Result<()> {
        if !source_file.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound, 
                format!("Source file '{}' does not exist", source_file.display())
            ));
        }

        self.command
            .arg(source_file);

        Ok(())
    }
    
    /// Add any arg to the compiler command
    pub fn add_arg(&mut self, arg: &str) {
        self.command.arg(arg);
    }

    /// Consumes itself and spawns the process, waits for its completion and returns the output
    pub fn execute(mut self) -> io::Result<Output> {
        let cmd_str = self.command.get_program().to_string_lossy();
        let args_str = self.command.get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<Cow<str>>>()
            .join(" ");
        
        println!("Executing: {} {}", cmd_str, args_str);
        self.command.output()
    }
}

pub fn print_output(output: &Output, indent: usize) {
    let indent_str = " ".repeat(indent);
    
    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        for line in stderr.lines() {
            eprintln!("{}{}", indent_str, line);
        }
    }
    
    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            println!("{}{}", indent_str, line);
        }
    }
}
