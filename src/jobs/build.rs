
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
    
    let compiler = Compiler::initialize(project_config.compiler.clone(), project_config.root_path.clone());

    for unit_name in unit_names {
        let unit = match project_config.find_unit(unit_name) {
            Some(unit) => unit,
            None => {
                println!("A unit with name '{}' does not exist in the project", unit_name);
                println!("Skipping...");
                continue;
            }
        };
        
        let source_files = unit.get_source_files().unwrap_or_else(|err| {
            eprintln!("An error occurred while trying to get unit '{}' source files: {}", unit_name, err.message());
            if let Some(cause) = err.cause() {
                eprintln!("  Cause: {}", cause);
            }
            process::exit(1);
        });
        
        let target = unit.get_target_information().unwrap_or_else(|err| {
            eprintln!("An error occurred while preparing the unit '{}' for build: {}", unit_name, err.message());
            if let Some(cause) = err.cause() {
                eprintln!("  Cause: {}", cause);
            }
            process::exit(1);
        });
        
        compiler.build(source_files, target);
    }

    println!("Copper project build finished");
}
