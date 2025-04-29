use std::path::Path;
use std::fs;
use std::process::exit;
use crate::config::project::{CopperProject, ProjectLanguage};

pub fn init(project_location: &Path, project_name: String, project_language: ProjectLanguage, generate_example: bool) {
    let project = CopperProject::init(project_location, project_name, project_language, generate_example);

    match project {
        Ok(path) => {
            println!("Created a new Copper project at {}", fs::canonicalize(path).unwrap().display());
        }
        Err(err) => {
            println!("Failed to initiate a new Copper project");
            eprintln!("{}", err);
            exit(1);
        }
    }

}