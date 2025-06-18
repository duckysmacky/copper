use std::process::{Command, Output};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Specifies the compiler-specific option flags
#[derive(Clone)]
pub struct CompilerCommandFlags {
    output: String,
    compile: String,
    include_directory: String,
    language: String
}

impl CompilerCommandFlags {
    pub fn new(
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
pub struct CompilerCommand {
    executable_name: String,
    compiler_flags: CompilerCommandFlags,
}

impl CompilerCommand {
    pub fn new(
        executable_name: &str,
        options_flags: CompilerCommandFlags
    ) -> Self {
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
pub struct CompilerCommandExecutor<'a> {
    command: Command,
    flags: &'a CompilerCommandFlags,
}

impl<'a> CompilerCommandExecutor<'a> {
    pub fn new(
        executable_name: &str, 
        flags: &'a CompilerCommandFlags
    ) -> Self {
        let command = Command::new(executable_name);

        CompilerCommandExecutor {
            command,
            flags,
        }
    }

    /// Specify the expected output file
    pub fn output(&mut self, output_file: &Path) -> io::Result<()> {
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
    pub fn compile(&mut self, source_file: &Path, language: Option<&str>, include_paths: &Option<Vec<PathBuf>>) -> io::Result<()> {
        if !source_file.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound, 
                format!("Source file '{}' does not exist", source_file.display())
            ));
        }

        if let Some(language) = language {
            self.command
                .arg(&self.flags.language)
                .arg(language);
        }

        if let Some(include_paths) = include_paths {
            for include_path in include_paths {
                if !include_path.exists() {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound, 
                        format!("Include path '{}' does not exist", include_path.display())
                    ));
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
    pub fn link(&mut self, object_files: &Vec<PathBuf>) -> io::Result<()> {
        for object_file in object_files {
            if !object_file.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound, 
                    format!("Object file '{}' does not exist", object_file.display())
                ));
            }
        }

        self.command
            .args(object_files);

        Ok(())
    }

    /// Consumes itself and spawns the process, waits for its completion and returns the output
    pub fn execute(mut self) -> io::Result<Output> {
        self.command.output()
    }
}