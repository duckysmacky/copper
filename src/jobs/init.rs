use std::path::{Path, PathBuf};
use std::{fs, io, process};
use std::rc::Rc;
use crate::project::{ProjectConfig, ProjectLanguage, ProjectCompiler, UnitConfig, UnitType};

/// Initiates a new copper project by generating a copper.yaml in the provided project location and
/// filling in all the required data
pub fn init(
    project_location: &Path,
    project_name: String, 
    project_language: ProjectLanguage, 
    generate_example: bool
) {
    let default_compiler = {
        if cfg!(windows) {
            ProjectCompiler::MSVC
        } else {
            match &project_language {
                ProjectLanguage::C => ProjectCompiler::GCC,
                ProjectLanguage::CPP => ProjectCompiler::GPP
            }
        }
    };
    
    let mut project = ProjectConfig::new(
        project_location.to_path_buf(),
        project_name,
        project_language,
        default_compiler,
    );
    
    if !fs::exists(project_location).unwrap_or(false) {
        if let Err(err) = fs::create_dir_all(project_location) {
            println!("Unable to create project directory '{}'", project_location.display());
            println!("\tCause: {}", err);
            process::exit(1);
        }
    }

    if generate_example {
        match add_example_config(&mut project) {
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
fn add_example_config(project: &mut Rc<ProjectConfig>) -> io::Result<()> {
    let src_dir = PathBuf::from("src");
    let unit_dir = src_dir.join("app");
    let include_dir = src_dir.join("include");

    /// Skip the error if it is an 'already exists' error (since it is not critical in this case)
    fn skip_already_exists(err: io::Error) -> io::Result<()> { if err.kind() == io::ErrorKind::AlreadyExists { Ok(()) } else { Err(err) } }
    fs::create_dir_all(project.root_path.join(&include_dir)).or_else(skip_already_exists)?;

    {
        match Rc::get_mut(project) {
            Some(project) => {
                project.global_include_paths.push(include_dir);
            },
            None => {
                eprintln!("Unable to add example include path to project");
                eprintln!("\tCause: multiple mutable references to project already exist");
                process::exit(1);
            }
        }
    }
    project.add_unit("example".to_string(), UnitType::Binary, unit_dir);

    Ok(())
}
