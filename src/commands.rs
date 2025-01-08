use std::env;
use std::path::{Path, PathBuf};
use clap::ArgMatches;
use crate::{jobs, project};

pub fn handle_init(matches: &ArgMatches) {
    let project_location = {
        let location = matches.get_one::<String>("location").unwrap();
        Path::new(location)
    };

    let project_name = match matches.get_one::<String>("name") {
        Some(name) => String::from(name),
        None => {
            let directory = if project_location == Path::new(".") {
                let current = env::current_dir().unwrap();
                let name = current.file_name().unwrap();
                name.to_os_string()
            } else {
                let name = project_location.file_name().unwrap();
                name.to_os_string()
            };
            String::from(directory.to_str().unwrap())
        }
    };

    jobs::init(project_location, project_name);
}

pub fn handle_build(matches: &ArgMatches) {
    let project_location = {
        let location = matches.get_one::<String>("location").unwrap();
        Path::new(location)
    };

    jobs::build(project_location);
}

pub fn handle_unit(matches: &ArgMatches) {
    let project_location = {
        let location = matches.get_one::<String>("location").unwrap();
        Path::new(location)
    };
    let unit_source = matches.get_one::<String>("source").unwrap();
    let unit_path = PathBuf::from(unit_source);
    let unit_name = Path::new(unit_source).file_name().unwrap().to_str().unwrap();
    let unit_type = {
        let type_name = matches.get_one::<String>("type").unwrap();
        project::UnitType::try_from(type_name.to_string()).unwrap()
    };

    jobs::add_unit(project_location, unit_name, unit_type.clone(), unit_path);
}