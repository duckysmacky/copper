use std::path::{Path, PathBuf};
use std::{fs, io, process};
use crate::config;
use crate::config::{
    project::{CopperProject, ProjectLanguage, ProjectCompiler},
    unit::{CopperUnit, UnitType}
};

/// Initiates a new copper project by generating a copper.toml in the provided project location and
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

    let mut include_paths = None;
    let mut units = Vec::new();

    if !fs::exists(&project_location).unwrap_or(false) {
        if let Err(err) = fs::create_dir_all(&project_location) {
            println!("Error creating project directory: {}", err);
            process::exit(1);
        }
    }

    if generate_example {
        match add_example_config(project_location, &mut units, &mut include_paths) {
            Ok(_) => println!("Successfully generated example project structure"),
            Err(err) => {
                println!("Error generating example project structure: {}", err);
                process::exit(1);
            }
        }
    }

    let project = CopperProject::new(
        project_location.to_path_buf(),
        project_name,
        project_language,
        default_compiler,
        include_paths,
        None,
        units,
    );

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
fn add_example_config(project_location: &Path, units: &mut Vec<CopperUnit>, include_paths: &mut Option<Vec<PathBuf>>) -> io::Result<()> {
    use config::default;
    
    let src_dir = default::SOURCE_DIRECTORY();
    let build_dir = default::BUILD_DIRECTORY();

    let unit_dir = src_dir.join("app");
    let include_dir = src_dir.join("include");
    let bin_dir = build_dir.join(default::BINARY_DIRECTORY());
    let obj_dir = build_dir.join(default::OBJECT_DIRECTORY());

    /// Skip the error if it is an 'already exists' error (since it is not critical in this case)
    fn skip_already_exists(err: io::Error) -> io::Result<()> { if err.kind() == io::ErrorKind::AlreadyExists { Ok(()) } else { Err(err) } }
    fs::create_dir_all(project_location.join(&bin_dir)).or_else(skip_already_exists)?;
    fs::create_dir_all(project_location.join(&obj_dir)).or_else(skip_already_exists)?;
    fs::create_dir_all(project_location.join(&include_dir)).or_else(skip_already_exists)?;

    *include_paths = Some(vec![include_dir]);

    units.push(CopperUnit::new(
        "example_app".to_string(),
        UnitType::Binary,
        unit_dir,
        bin_dir,
        obj_dir,
        None,
        None,
    ));

    Ok(())
}
