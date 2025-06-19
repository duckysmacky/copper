use std::path::Path;
use std::process;
use crate::compiler::Compiler;
use crate::config::{ProjectConfig, Error, Result};

pub fn build<'a>(unit_names: Option<impl Iterator<Item = &'a String>>, project_location: &Path) {
    let project = match ProjectConfig::import(project_location) {
        Ok(project) => project,
        Err(err) => {
            eprintln!("Unable to import project: {}", err);
            process::exit(1);
        }
    };
    
    if let Err(err) = build_units(&project, unit_names) {
        println!("Unable to build project");
        eprintln!("{}", err);
        process::exit(1);
    }

    println!("Copper project build finished");
}

/// Builds specifies units (by name) or the whole project (all units)
fn build_units<'a>(project: &ProjectConfig, unit_names: Option<impl Iterator<Item = &'a String>>) -> Result<()> {
    let unit_names = match unit_names {
        None => project.get_unit_names(),
        Some(names) => names.collect(),
    };
    
    if unit_names.is_empty() {
        return Err(Error::NoUnits)
    }
    
    let compiler_options = project.get_compiler_options();
    let compiler = Compiler::initialize(project.compiler.clone(), compiler_options);

    for unit_name in unit_names {
        
        let target = match project.find_unit(unit_name) {
            Some(unit) => unit.get_target_information(&project),
            None => return Err(Error::UnitNotFound(unit_name.to_string())),
        };
        
        if let None = target {
            continue
        }

        compiler.build(target.unwrap());
    }

    Ok(())
}