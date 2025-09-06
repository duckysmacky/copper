use std::{env, fs, io, process};
use std::path::{Path, PathBuf};
use clap::ArgMatches;
use crate::project::{CopperProject, ProjectCompiler, ProjectLanguage, UnitType};

/// Handles the main project initialization logic
/// 
/// Initiates a new copper project by generating a copper.yaml in the provided
/// project location and filling in all the required data
pub fn handle_init(matches: &ArgMatches) {
    let project_location = matches.get_one::<PathBuf>("location").unwrap();

    let project_language = {
        let language_str = matches.get_one::<String>("language").unwrap();
        ProjectLanguage::try_from(language_str.to_string()).unwrap()
    };
    
    let project_compiler = {
        let compiler_str = matches.get_one::<String>("compiler").unwrap();
        ProjectCompiler::try_from(compiler_str.to_string()).unwrap()
    };

    let project_name = match matches.get_one::<String>("name") {
        Some(name) => String::from(name),
        None => {
            let directory = if project_location == Path::new("../..") {
                let current = env::current_dir().unwrap();
                let name = current.file_name().unwrap();
                name.to_os_string()
            } else {
                let name = project_location.file_name().unwrap();
                name.to_os_string()
            };
            String::from(directory.to_string_lossy())
        }
    };

    let project = CopperProject::new(
        project_location.to_path_buf(),
        project_name,
        project_language,
        project_compiler
    );

    let generate_example = matches.get_flag("example") && !matches.get_flag("minimal");
        
    if !fs::exists(project_location).unwrap_or(false) {
        if let Err(err) = fs::create_dir_all(project_location) {
            println!("Unable to create project directory '{}'", project_location.display());
            println!("\tCause: {}", err);
            process::exit(1);
        }
    }

    if generate_example {
        println!("Generating example project structure...");
        match add_example_config(&project) {
            Ok(_) => println!("Successfully generated example project structure"),
            Err(err) => {
                println!("Unable to generate example project structure");
                println!("\tCause: {}", err);
                process::exit(1);
            }
        }
    }

    match project.save(project_location) {
        Ok(_) => {
            let cannon_path = project_location.canonicalize().unwrap_or(project_location.to_path_buf());
            println!("Created a new Copper project at '{}'", cannon_path.display());
        },
        Err(err) => {
            eprintln!("Unable to initialize project: {}", err);
            process::exit(1);
        }
    }
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
