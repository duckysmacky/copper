use std::path::{Path, PathBuf};
use std::{fs, io, process};
use crate::config::{
    project::{CopperProject, ProjectLanguage, ProjectCompiler},
    unit::{CopperUnit, UnitType}
};

pub fn init(
    project_location: &Path,
    project_name: String, 
    project_language: ProjectLanguage, 
    generate_example: bool
) {
    let default_compiler = {
        if cfg!(target_os = "windows") {
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

    if generate_example {
        match add_example_config(project_location, &mut units, &mut include_paths) {
            Ok(_) => println!("Sucessfully generated example project structure"),
            Err(err) => println!("Error generating example project structure: {}", err)
        };
    }

    let project = CopperProject::new(
        project_name,
        project_language,
        default_compiler,
        include_paths,
        units,
        project_location.to_path_buf()
    );

    match project.save(project_location) {
        Ok(_) => {
            println!("Created a new Copper project at {}", fs::canonicalize(project_location).unwrap().display());
        }
        Err(err) => {
            eprintln!("Unable to initialize project: {}", err);
            process::exit(1);
        }
    }
}

fn add_example_config(project_location: &Path, units: &mut Vec<CopperUnit>, include_paths: &mut Option<Vec<PathBuf>>) -> io::Result<()> {
    let src_dir = project_location.join("..");
    let build_dir = project_location.join("build");
    
    // TODO: add skip for 'already exists' io error
    fs::create_dir_all(src_dir.join("include"))?;
    fs::create_dir_all(build_dir.join("bin"))?;
    fs::create_dir_all(build_dir.join("obj"))?;

    *include_paths = Some(vec![src_dir.join("include")]);

    units.push(CopperUnit::new(
        "example".to_string(),
        UnitType::Binary,
        PathBuf::from("src/"),
        PathBuf::from("build/bin"),
        PathBuf::from("build/obj")
    ));

    Ok(())
}
