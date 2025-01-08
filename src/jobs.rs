use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Write;
use crate::project::{CopperProject, UnitType, PROJECT_FILE_NAME};

pub fn init(project_dir: &Path, project_name: String) {
    let file_path = project_dir.join(PROJECT_FILE_NAME);
    let mut file = File::create_new(&file_path).expect("File already exists");

    let project = CopperProject::init(project_name, project_dir);
    let toml_data = toml::to_string(&project).expect("Invalid project file");

    file.write_all(toml_data.as_bytes()).expect("Unable to write to file");
    file.flush().expect("Unable to close the file");

    println!("Created a new Copper project at {}", fs::canonicalize(file_path).unwrap().display());
}

pub fn build(project_location: &Path) {
    let project = CopperProject::import(project_location);
    
    project.build();
    println!("Copper project build finished");
}

pub fn add_unit(project_location: &Path, unit_name: &str, unit_type: UnitType, unit_source: PathBuf) {
    let mut project = CopperProject::import(project_location);

    project.add_unit(unit_name.to_string(), unit_type.clone(), unit_source);
    project.save(project_location);
    println!("Successfully added unit \"{}\"", unit_name);
}