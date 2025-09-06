use std::path::Path;
use std::process;
use crate::compiler::Compiler;
use crate::project::{CopperProject};

// TODO: refactor build process
pub fn build<'a>(unit_names: Option<impl Iterator<Item = &'a String>>, project_location: &Path) {
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
    
    build_units(&project, unit_names);
    println!("Copper project build finished");
}

/// Builds specifies units (by name) or the whole project (all units)
fn build_units<'a>(project: &CopperProject, unit_names: Option<impl Iterator<Item = &'a String>>) {
    let project_config = project.get_config();
    
    let unit_names = match unit_names {
        None => project_config.get_unit_names(),
        Some(names) => names.collect(),
    };
    
    if unit_names.is_empty() {
        println!("The project has no units to build");
        return;
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
            Ok(target) => compiler.build(target),
            Err(err) => {
                eprintln!("An error occurred while preparing the unit '{}' for build: {}", unit_name, err.message());
                if let Some(cause) = err.cause() {
                    eprintln!("  Cause: {}", cause);
                }
                process::exit(1);
            }
        }
    }
}