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
    compiler: ProjectCompiler,
    language: ProjectLanguage,
}

impl Compiler {
    /// Returns a specific compiler instance based on the selected project compiler
    pub fn initialize(project_compiler: ProjectCompiler, options: CompilerOptions) -> Self {
        if !util::check_if_available(&project_compiler) {
            eprintln!("Unsupported compiler specified");
            eprintln!("  Cause: Compiler not avaliable on the current system");
            process::exit(1);
        }
        
        let compiler_flags = match project_compiler {
            ProjectCompiler::GCC => gcc::FLAGS,
            _ => unimplemented!()
        };

        Compiler {
            command: CompilerCommand::new(
                project_compiler.executable_name(),
                compiler_flags,
                options.root_path,
                options.include_paths.unwrap_or(Vec::new()),
                options.additional_flags.map_or(Vec::new(), |flags| flags.split_whitespace().map(String::from).collect()), 
            ),
            compiler: project_compiler,
            language: options.target_language,
        }
    }
    
    pub fn build(&self, target: TargetInformation) {
        let object_files = target.source_files.iter().map(|file| {
            self.compile(file, &target).unwrap_or_else(|err| {
                eprintln!("Compilation failed for target '{}'", &target.name);
                eprintln!("An error accured while trying to compile '{}'", file.display());
                eprintln!("  Cause: {}", err);
                process::exit(1);
            })
        }).collect();
        
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
        let mut command_executor = self.command.executor()?;
        
        command_executor.set_language(&self.language);
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

        Ok(object_file)
    }

    /// Links compiled object files to the output file
    fn link_objects(&self, target: &TargetInformation, object_files: Vec<PathBuf>) -> io::Result<()> {
        let mut command_executor = self.command.executor()?;
        
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

        Ok(())
    }
    
}

/// Options for configuring compiler's behaviour and supplying persistent attributes for the whole
/// duration of the process
pub struct CompilerOptions {
    /// Relative path to the project root from the process's location to correctly supply
    /// path-based compiler arguments
    root_path: PathBuf,
    /// The main language which is going to be used for compilation
    target_language: ProjectLanguage,
    /// Additional paths which are going to be included
    include_paths: Option<Vec<PathBuf>>,
    /// Additional flags which are going to be supplied to the compiler
    additional_flags: Option<String>,
}

impl CompilerOptions {
    pub fn new(
        root_path: PathBuf,
        target_language: ProjectLanguage,
        include_paths: Option<Vec<PathBuf>>,
        additional_flags: Option<String>,
    ) -> Self {
        CompilerOptions {
            root_path,
            target_language,
            include_paths,
            additional_flags,
        }
    }
}

/// Required information about the target which is used to perform target-specific actions
pub struct TargetInformation {
    /// Target output name
    name: String,
    /// Type of target
    r#type: UnitType,
    /// Target's source files
    source_files: Vec<PathBuf>,
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
        r#type: UnitType,
        source_files: Vec<PathBuf>,
        output_directory: PathBuf,
        intermediate_directory: PathBuf,
        include_paths: Option<Vec<PathBuf>>,
        additional_args: Option<String>,
    ) -> Self {
        TargetInformation {
            name,
            r#type,
            source_files,
            output_directory,
            intermediate_directory,
            include_paths: include_paths.unwrap_or(Vec::new()),
            additional_args: additional_args.map_or(Vec::new(), |a| a.split_whitespace().map(String::from).collect()),
        }
    }
}
