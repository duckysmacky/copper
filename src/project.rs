use std::fs::File;
use std::io::Write;
use std::path::{self, Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CopperProject {
    pub general: CopperGeneral
}

#[derive(Serialize, Deserialize)]
struct CopperGeneral {
    pub name: String
}

pub fn init(dir_path: &Path, project_name: String) {
    let file_path = dir_path.join("copper.toml");
    let mut file = File::create_new(file_path).expect("File already exists");

    let project_file = CopperProject {
        general: CopperGeneral {
            name: project_name.to_string()
        },
    };
    let toml_data = toml::to_string(&project_file).expect("Invalid project file");

    file.write_all(toml_data.as_bytes()).expect("Unable to write to file");
    file.flush().expect("Unable to close the file");

    println!("Created a new Copper project at {}", path::absolute(dir_path).unwrap().display());
}