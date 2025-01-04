use std::env;
use std::path::Path;
use clap::ArgMatches;
use crate::project;

pub fn handle_init(matches: &ArgMatches) {
    let directory_path = match matches.get_one::<String>("directory") {
        Some(path) => Path::new(path),
        None => Path::new(".")
    };

    let project_name = match matches.get_one::<String>("name") {
        Some(name) => String::from(name),
        None => {
            let directory = if directory_path == Path::new(".") {
                let current = env::current_dir().unwrap();
                let name = current.file_name().unwrap();
                name.to_os_string()
            } else {
                let name = directory_path.file_name().unwrap();
                name.to_os_string()
            };
            String::from(directory.to_str().unwrap())
        }
    };

    project::init(directory_path, project_name);
}