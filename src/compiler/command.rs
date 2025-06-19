use std::process::{Command, Output};
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::borrow::Cow;
use std::ffi::OsString;
use crate::config::ProjectLanguage;

/// Specifies the compiler-specific option flags
pub struct CompilerCommandFlags {
    pub output: &'static str,
    pub compile: &'static str,
    pub include: &'static str,
    pub language: &'static str,
}

/// Wrapper for the compiler command executor
pub struct CompilerCommand {
    executable_name: String,
    command_flags: CompilerCommandFlags,
    root_relative_path: PathBuf,
    include_paths: Vec<PathBuf>,
    additional_args: Vec<String>,
}

impl CompilerCommand {
    pub fn new(
        executable_name: String,
        command_flags: CompilerCommandFlags,
        root_relative_path: PathBuf,
        include_paths: Vec<PathBuf>,
        additional_args: Vec<String>,
    ) -> Self {
        CompilerCommand {
            executable_name,
            command_flags,
            root_relative_path,
            include_paths,
            additional_args,
        }
    }

    /// Initiates a new Executor to use
    pub fn executor<'a>(&self) -> io::Result<CompilerCommandExecutor> {
        let mut executor = CompilerCommandExecutor::new(
            &self.executable_name,
            &self.command_flags,
            &self.root_relative_path,
        );
        
        self.include_paths.iter().try_for_each(|p| executor.add_include_path(p))?;
        self.additional_args.iter().for_each(|a| executor.add_arg(a));
        
        Ok(executor)
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