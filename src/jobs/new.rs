use std::path::{Path, PathBuf};
use std::process::exit;
use crate::config::project::CopperProject;
use crate::config::unit::UnitType;

pub fn new_unit(project_location: &Path, unit_name: &str, unit_type: UnitType, unit_source: PathBuf) {
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