use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{self, Path, PathBuf};
use std::process::{Command, Output};
use serde::{Deserialize, Serialize};

const COPPER_FILE_NAME: &str = "copper.toml";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CopperProject {
    name: String,
    compiler: String,
    unit: Option<CopperUnit>
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CopperUnit {
    name: String,
    directory: PathBuf,
    output: Option<PathBuf>
}

impl CopperProject {
    pub fn init(name: String) -> CopperProject {
        CopperProject {
            name,
            compiler: "gcc".to_string(),
            unit: None,
        }
    }

    pub fn build(&self, project_directory: &Path) {
        let unit = self.unit.clone().expect("No unit to build");
        let unit_dir = project_directory.join(unit.directory);
        let paths = fs::read_dir(&unit_dir).expect("Unit dir doesn't exist");

        let mut source_file_paths: Vec<PathBuf> = Vec::new();
        for path in paths {
            let file = path.unwrap().path();
            if let Some(ext) = file.extension() {
                if ext.eq("c") {
                    source_file_paths.push(file);
                }
            }
        }

        let mut output_dir = project_directory.to_path_buf();
        if let Some(output) = unit.output {
            output_dir.push(output);
        }
        let output_file = output_dir.join(unit.name);

        eprintln!("Compiling {:?} into {}", source_file_paths, output_file.display());
        self.compile(source_file_paths, &output_file);
    }

    fn compile(&self, sources: Vec<PathBuf>, output: &Path) -> Output {
        Command::new(&self.compiler)
            .args(sources)
            .arg("-o")
            .arg(output)
            .output()
            .expect("Unable to compile")
    }
}

pub fn init(project_dir: &Path, project_name: String) {
    let file_path = project_dir.join(COPPER_FILE_NAME);
    let mut file = File::create_new(file_path).expect("File already exists");

    let project = CopperProject::init(project_name);
    let toml_data = toml::to_string(&project).expect("Invalid project file");

    file.write_all(toml_data.as_bytes()).expect("Unable to write to file");
    file.flush().expect("Unable to close the file");

    println!("Created a new Copper project at {}", path::absolute(project_dir).unwrap().display());
}

pub fn build(project_dir: &Path) {
    let file_path = project_dir.join(COPPER_FILE_NAME);
    let mut file = File::open(file_path).expect("File not found");

    let mut file_data = String::new();
    file.read_to_string(&mut file_data).expect("Unable to read the file");
    let project: CopperProject = toml::from_str(&file_data).expect("Unable to deserialize");

    project.build(project_dir);
    println!("Successfully built \"{}\"", &project.name);
}