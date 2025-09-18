use std::path::{Path, PathBuf};
use std::process;
use clap::ArgMatches;

use crate::project::{CopperProject, UnitType};

pub fn handle_new(matches: &ArgMatches) {
    let project_location = matches.get_one::<PathBuf>("location").unwrap();
 
    if let Some(matches) = matches.subcommand_matches("unit") {
        new_unit(project_location, matches);
    }
}

fn new_unit(project_location: &Path, matches: &ArgMatches) {
    let project = CopperProject::import(project_location).unwrap_or_else(|err| {
        eprintln!("Unable to import project file. Perhaps it does not exist");
        eprintln!("  Cause: {}", err);
        process::exit(1);
    });
    
    let unit_source = matches.get_one::<PathBuf>("source").unwrap();
    
    if !unit_source.is_dir() {
        eprintln!("Invalid unit source '{}' provided", unit_source.display());
        eprintln!("  Cause: The provided unit source is not a directory");
        process::exit(1);
    }
    
    if !unit_source.exists() {
        eprintln!("Invalid unit source '{}' provided", unit_source.display());
        eprintln!("  Cause: The provided unit source does not exist");
        process::exit(1);
    }
    
    let unit_name = match matches.get_one::<String>("name") {
        Some(name) => name.to_string(),
        None => {
            let dir_name = unit_source.file_name().unwrap_or_else(|| {
                eprintln!("Unable to determine unit source directory name");
                process::exit(1);
            });
            dir_name.to_string_lossy().to_string()
        }
    };

    let unit_type = {
        let type_str = matches.get_one::<String>("type").unwrap();
        UnitType::try_from(type_str.to_string()).unwrap()
    };

    project.add_unit(unit_name.to_string(), unit_type.clone(), unit_source.clone());

    if let Err(err) = project.save(project_location) {
        eprintln!("Unable to save updated project file");
        eprintln!("  Cause: {}", err);
        process::exit(1);
    }

    println!("Successfully added a new unit '{}' to the project", unit_name);
}
