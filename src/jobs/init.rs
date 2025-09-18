use std::{env, fs, io, process};
use std::path::PathBuf;
use clap::ArgMatches;
use crate::project::{CopperProject, ProjectCompiler, ProjectLanguage, UnitType};

/// Handles the main project initialization logic
/// 
/// Initiates a new copper project by generating a copper.yaml in the provided
/// project location and filling in all the required data
pub fn handle_init(matches: &ArgMatches) {
    let project_location = matches.get_one::<PathBuf>("location").unwrap();
    
    if !project_location.is_dir() {
        eprintln!("Invalid project location '{}' provided", project_location.display());
        eprintln!("  Cause: The provided project location is not a directory");
        process::exit(1);
    }
    
    if !project_location.exists() {
        eprintln!("Invalid project location '{}' provided", project_location.display());
        eprintln!("  Cause: The provided project location does not exist");
        process::exit(1);
    }
    
    if let Ok(project) = CopperProject::import(project_location) {
        eprintln!("Invalid project location '{}' provided", project_location.display());
        eprintln!("  Cause: A Copper project '{}' already exists at the provided location", project.get_config().name);
        process::exit(1);
    }

    let project_language = {
        let language_str = matches.get_one::<String>("language").unwrap();
        ProjectLanguage::try_from(language_str.to_string()).unwrap()
    };
    
    let project_compiler = {
        let compiler_str = matches.get_one::<String>("compiler").unwrap();
        ProjectCompiler::try_from(compiler_str.to_string()).unwrap()
    };

    let project_name = match matches.get_one::<String>("name") {
        Some(name) => name.to_string(),
        None => {
            match project_location.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => {
                    let current_dir = env::current_dir().unwrap_or_else(|err| {
                        eprintln!("Unable to determine current working directory");
                        eprintln!("  Cause: {}", err);
                        process::exit(1);
                    });
                    
                    let dir_name = current_dir.file_name().unwrap_or_else(|| {
                        eprintln!("Unable to determine current working directory name");
                        process::exit(1);
                    });
                    
                    dir_name.to_string_lossy().to_string()
                }
            } 
        }
    };

    let project = CopperProject::new(
        project_location.to_path_buf(),
        project_name,
        project_language,
        project_compiler
    );

    if !fs::exists(project_location).unwrap_or(false) {
        if let Err(err) = fs::create_dir_all(project_location) {
            eprintln!("Unable to create project directory '{}'", project_location.display());
            eprintln!("  Cause: {}", err);
            process::exit(1);
        }
    }

    let generate_example = matches.get_flag("example") && !matches.get_flag("minimal");
    
    if generate_example {
        println!("Generating example project structure...");
        
        add_example_config(&project).unwrap_or_else(|err| {
            eprintln!("Unable to generate example project structure");
            eprintln!("  Cause: {}", err);
            process::exit(1);
        });
        
        println!("Generated example project structure");
    }
    
    project.save(project_location).unwrap_or_else(|err| {
        eprintln!("Unable to save initialized project file");
        eprintln!("  Cause: {}", err);
        process::exit(1);
    });

    let cannon_path = project_location.canonicalize().unwrap_or(project_location.to_path_buf());
    println!("Successfully created a new Copper project at '{}'", cannon_path.display());
}

/// Generates an example project configuration. Creates default directories and appends example
/// unit and include path to project data
fn add_example_config(project: &CopperProject) -> io::Result<()> {
    let src_dir = PathBuf::from("src");
    let unit_dir = src_dir.join("app");
    let include_dir = src_dir.join("include");

    // skip the error if it is an 'already exists' error (since it is not critical in this case)
    let skip_already_exists: fn(io::Error) -> io::Result<()> = |err| if err.kind() == io::ErrorKind::AlreadyExists { Ok(()) } else { Err(err) };
    project.with_config(|config| {
        fs::create_dir_all(config.root_path.join(&include_dir)).or_else(skip_already_exists)
    }).or_else(skip_already_exists)?;

    project.with_config_mut(|config| {
        config.global_include_paths.push(include_dir.clone());
    });
    project.add_unit("example".to_string(), UnitType::Binary, unit_dir);

    Ok(())
}
