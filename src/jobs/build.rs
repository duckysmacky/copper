
use std::path::PathBuf;
use std::process;
use clap::ArgMatches;
use crate::compiler::Compiler;
use crate::project::{CopperProject};

/// Handles the project build process
pub fn handle_build(matches: &ArgMatches) {
    let project_location = matches.get_one::<PathBuf>("location").unwrap();

    let project = match CopperProject::import(project_location) {
        Ok(project) => project,
        Err(err) => {
            eprintln!("An error occurred while importing the project: {}", err.message());
            if let Some(cause) = err.cause() {
                eprintln!("  Cause: {}", cause);
            }
            process::exit(1);
        }
    };
    
    let project_config = project.get_config();
    
    let unit_names = matches.get_many::<String>("units")
        .map(|units| units.into_iter().collect())
        .unwrap_or(project_config.get_unit_names());

    if unit_names.is_empty() {
        eprintln!("There are no units to build");
        process::exit(1);
    }
    
    let compiler_options = project.get_compiler_options();
    let compiler = Compiler::initialize(project_config.compiler.clone(), compiler_options);

    for unit_name in unit_names {
        let target = match project_config.find_unit(unit_name) {
            Some(unit) => unit.get_target_information(),
            None => {
                println!("A unit with name '{}' does not exist in the project", unit_name);
                println!("Skipping...");
                continue;
            }
        };
        
        match target {
            Ok(target) => {
                println!("Unit '{}' build started", unit_name);
                compiler.build(target);
                println!("Unit '{}' build finished", unit_name);
            }
            Err(err) => {
                eprintln!("An error occurred while preparing the unit '{}' for build: {}", unit_name, err.message());
                if let Some(cause) = err.cause() {
                    eprintln!("  Cause: {}", cause);
                }
                process::exit(1);
            }
        }
    }

    println!("Copper project build finished");
}
