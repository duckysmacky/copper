use std::path::{Path, PathBuf};
use std::process::exit;
use clap::ArgMatches;

use crate::project::{CopperProject, UnitType};

pub fn handle_new(matches: &ArgMatches) {
    let project_location = matches.get_one::<PathBuf>("location").unwrap();
 
    if let Some(matches) = matches.subcommand_matches("unit") {
        new_unit(project_location, matches);
    }
}

fn new_unit(project_location: &Path, matches: &ArgMatches) {
    // TODO: add validation for existing unit directory
    let unit_source = matches.get_one::<PathBuf>("source").unwrap();
    
    let unit_name = match matches.get_one::<String>("name") {
        Some(name) => String::from(name),
        None => {
            let directory = unit_source.file_name().unwrap().to_os_string();
            String::from(directory.to_string_lossy())
        }
    };

    let unit_type = {
        let type_str = matches.get_one::<String>("type").unwrap();
        UnitType::try_from(type_str.to_string()).unwrap()
    };

    match CopperProject::import(project_location) {
        Ok(project) => {
            project.add_unit(unit_name.to_string(), unit_type.clone(), unit_source.clone());

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
