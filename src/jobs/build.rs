use std::path::Path;
use std::process::exit;
use crate::config::project::CopperProject;

pub fn build<'a>(units: Option<impl Iterator<Item = &'a String>>, project_location: &Path) {
    let project = CopperProject::import(project_location);

    if let Err(err) = project {
        println!("Unable to import project");
        eprintln!("{}", err);
        exit(1);
    }

    let project = project.unwrap();

    if let Err(err) = project.build(units) {
        println!("Unable to build project");
        eprintln!("{}", err);
        exit(1);
    }

    println!("Copper project build finished");
}