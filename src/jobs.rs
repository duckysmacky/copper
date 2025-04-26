use std::path::{Path, PathBuf};
use std::fs;
use std::process::exit;
use crate::project::{CopperProject, CopperProjectLanguage, UnitType};

pub fn init(project_location: &Path, project_name: String, project_language: CopperProjectLanguage, generate_example: bool) {
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

pub fn build(project_location: &Path) {
    let project = CopperProject::import(project_location);

    match project {
        Ok(project) => {
            if let Err(err) = project.build() {
                println!("Unable to build project");
                eprintln!("{}", err);
                exit(1);
            }
        }
        Err(err) => {
            println!("Unable to import project");
            eprintln!("{}", err);
            exit(1);
        }
    }

    println!("Copper project build finished");
}

pub fn add_unit(project_location: &Path, unit_name: &str, unit_type: UnitType, unit_source: PathBuf) {
    let project = CopperProject::import(project_location);

    match project {
        Ok(mut project) => {
            project.add_unit(unit_name.to_string(), unit_type.clone(), unit_source);

            if let Err(err) = project.save(project_location) {
                println!("Unable to save project file");
                eprintln!("{}", err);
                exit(1);
            }
        },
        Err(err) => {
            println!("Unable to import project file");
            eprintln!("{}", err);
            exit(1);
        }
    }

    println!("Successfully added unit \"{}\"", unit_name);
}