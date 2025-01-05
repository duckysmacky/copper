use std::env;
use std::path::Path;
use clap::ArgMatches;
use crate::project;

pub fn handle_init(matches: &ArgMatches) {
    let project_dir = Path::new(matches.get_one::<String>("directory").unwrap());

    let project_name = match matches.get_one::<String>("name") {
        Some(name) => String::from(name),
        None => {
            let directory = if project_dir == Path::new(".") {
                let current = env::current_dir().unwrap();
                let name = current.file_name().unwrap();
                name.to_os_string()
            } else {
                let name = project_dir.file_name().unwrap();
                name.to_os_string()
            };
            String::from(directory.to_str().unwrap())
        }
    };

    project::init(project_dir, project_name);
}

pub fn handle_build(matches: &ArgMatches) {
    let project_dir = Path::new(matches.get_one::<String>("directory").unwrap());

    project::build(project_dir);
}