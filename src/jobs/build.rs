use std::path::Path;
use std::process;
use crate::compiler::{self, Compiler, CompilerError};
use crate::config::{ProjectConfig, UnitConfig, Error, Result};

pub fn build<'a>(units: Option<impl Iterator<Item = &'a String>>, project_location: &Path) {
    let project = match ProjectConfig::import(project_location) {
        Ok(project) => project,
        Err(err) => {
            eprintln!("Unable to import project: {}", err);
            process::exit(1);
        }
    };
    
    if let Err(err) = build_units(&project, units) {
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

    for unit_name in unit_names {
        match project.find_unit(unit_name) {
            Some(unit) => build_unit(project, unit),
            None => return Err(Error::UnitNotFound(unit_name.to_string()))
        }
    }

    Ok(())
}

/// Builds a unit by first getting compile options and then running a compiler using those
/// options
fn build_unit(project: &ProjectConfig, unit: &UnitConfig) {
    let compile_options = match unit.get_compile_options(project) {
        Ok(options) => options,
        Err(err) => {
            eprintln!("Unable to build project: {}", err);
            process::exit(1);
        }
    };

    let compiler = compiler::get_compiler(&project.compiler, compile_options);

    match compiler.compile() {
        Ok(_) => println!("Successfully compiled '{}'", unit.name),
        Err(err) => {
            eprintln!("Error compiling '{}':\n{}", unit.name, err.display());
            process::exit(1);
        }
    }

    match compiler.link() {
        Ok(_) => println!("Successfully linked '{}'", unit.name),
        Err(err) => {
            eprintln!("Error linking '{}':\n{}", unit.name, err.display());
            process::exit(1);
        }
    }

    println!("Successfully built '{}'", unit.name);
}