use std::path::Path;
use std::process;
use crate::compiler::Compiler;
use crate::project::{CopperProject, Error, Result};

pub fn build<'a>(unit_names: Option<impl Iterator<Item = &'a String>>, project_location: &Path) {
    let project = match CopperProject::import(project_location) {
        Ok(project) => project,
        Err(err) => {
            eprintln!("Unable to import project");
            eprintln!("\tCause: {}", err);
            process::exit(1);
        }
    };
    
    if let Err(err) = build_units(&project, unit_names) {
        eprintln!("Unable to build project");
        eprintln!("\tCause: {}", err);
        process::exit(1);
    }

    println!("Copper project build finished");
}

/// Builds specifies units (by name) or the whole project (all units)
fn build_units<'a>(project: &CopperProject, unit_names: Option<impl Iterator<Item = &'a String>>) -> Result<()> {
    let project_config = project.get_config();
    
    let unit_names = match unit_names {
        None => project_config.get_unit_names(),
        Some(names) => names.collect(),
    };
    
    if unit_names.is_empty() {
        return Err(Error::NoUnits)
    }
    
    let compiler_options = project.get_compiler_options();
    let compiler = Compiler::initialize(project_config.compiler.clone(), compiler_options);

    for unit_name in unit_names {
        let target = match project_config.find_unit(unit_name) {
            Some(unit) => unit.get_target_information(),
            None => return Err(Error::UnitNotFound(unit_name.to_string())),
        };
        
        if let None = target {
            continue
        }

        compiler.build(target.unwrap());
    }

    Ok(())
}