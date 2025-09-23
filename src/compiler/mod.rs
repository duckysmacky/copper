use std::{io, process};
use std::path::PathBuf;
use command::CompilerCommand;
use crate::project::{ProjectCompiler, ProjectLanguage, UnitType};

mod gcc;
mod util;
mod command;
mod error;

/// An instance of a generic compiler which is responsible for building, compiling and linking
/// project files
pub struct Compiler {
    command: CompilerCommand,
}

impl Compiler {
    /// Returns a specific compiler instance based on the selected project compiler
    pub fn initialize(project_compiler: ProjectCompiler, root_path: PathBuf) -> Self {
        if !util::check_if_available(&project_compiler) {
            eprintln!("Unsupported compiler specified");
            eprintln!("  Cause: Compiler not available on the current system");
            process::exit(1);
        }
        
        let compiler_flags = match project_compiler {
            ProjectCompiler::GCC => gcc::FLAGS,
            _ => {
                eprintln!("Unsupported compiler specified");
                eprintln!("  Cause: This compiler is not yet implemented");
                unimplemented!()
            }
        };

        Compiler {
            command: CompilerCommand::new(
                project_compiler.executable_name(),
                compiler_flags,
                root_path
            ),
        }
    }
    
    pub fn build(&self, source_files: Vec<PathBuf>, target: TargetInformation) {
        println!("Building target '{}'...", target.name);
        
        let object_files = source_files.iter().map(|file| {
            println!("Compiling '{}'...", file.display());
            self.compile(file, &target).unwrap_or_else(|err| {
                eprintln!("Compilation failed for target '{}'", &target.name);
                eprintln!("An error occurred while trying to compile '{}'", file.display());
                eprintln!("  Cause: {}", err);
                process::exit(1);
            })
        }).collect();
        
        println!("Linking object files for target '{}'...", target.name);
        if let Err(err) = self.link_objects(&target, object_files) {
            eprintln!("Linking failed for target '{}'", &target.name);
            eprintln!("  Cause: {}", err);
            process::exit(1);
        }
        
        println!("Build finished successfully for target '{}'", target.name);
    }

    /// Compiles target's source file into object files with the same name. Returns a path to the
    /// object file
    fn compile(&self, source_file: &PathBuf, target: &TargetInformation) -> io::Result<PathBuf> {
        let mut command_executor = self.command.executor();
        
        command_executor.set_language(&target.language);
        command_executor.set_compile_flag();

        target.include_paths.iter().try_for_each(|p| command_executor.add_include_path(p))?;
        target.additional_args.iter().for_each(|arg| command_executor.add_arg(arg));
        
        let object_file = {
            let mut file = target.intermediate_directory.join(&source_file.file_name().unwrap());
            file.set_extension("o");
            file
        };
        
        command_executor.set_output_file(&object_file)?;
        command_executor.add_input_file(&source_file)?;
        
        let output = command_executor.execute()?;

        if !output.status.success() {
            eprintln!("Compilation failed for target '{}'", &target.name);
            eprintln!("Unable to compile '{}' ({})", source_file.display(), output.status);
            command::print_output(&output, 4);
            let exit_code = output.status.code().unwrap_or(1);
            process::exit(exit_code);
        }

        println!("Successfully compiled '{}' to '{}'", source_file.display(), object_file.display());
        Ok(object_file)
    }

    /// Links compiled object files to the output file
    fn link_objects(&self, target: &TargetInformation, object_files: Vec<PathBuf>) -> io::Result<()> {
        let mut command_executor = self.command.executor();
        
        object_files.iter().try_for_each(|file| command_executor.add_input_file(file))?;
        
        let output_file = target.output_directory.join(&target.name);
        command_executor.set_output_file(&output_file)?;

        let output = command_executor.execute()?;
        
        if !output.status.success() {
            eprintln!("Linking failed for target '{}'", &target.name);
            command::print_output(&output, 4);
            let exit_code = output.status.code().unwrap_or(1);
            process::exit(exit_code);
        }

        println!("Successfully linked object files to '{}'", output_file.display());
        Ok(())
    }
}

/// Required information about the target which is used to perform target-specific actions
pub struct TargetInformation {
    /// Target output name
    name: String,
    /// The main language which is going to be used for compilation
    language: ProjectLanguage,
    /// Type of target
    r#type: UnitType,
    /// The location in which target will be output
    output_directory: PathBuf,
    /// The location for holding target's intermediate object
    intermediate_directory: PathBuf,
    /// Target-specific include paths
    include_paths: Vec<PathBuf>,
    /// Target-specific additional flags
    additional_args: Vec<String>,
}

impl TargetInformation {
    pub fn new(
        name: String,
        language: ProjectLanguage,
        r#type: UnitType,
        output_directory: PathBuf,
        intermediate_directory: PathBuf,
        include_paths: Vec<PathBuf>,
        additional_args: Vec<String>,
    ) -> Self {
        TargetInformation {
            name,
            language,
            r#type,
            output_directory,
            intermediate_directory,
            include_paths,
            additional_args,
        }
    }
}
