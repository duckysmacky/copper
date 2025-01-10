use std::env;
use std::path::{Path, PathBuf};
use clap::ArgMatches;
use crate::{jobs, project};

pub fn handle_init(matches: &ArgMatches) {
    let project_language = {
        let language_str = matches.get_one::<String>("language").unwrap();
        // Safe to unwrap as we already checked for valid enum strings
        project::CopperProjectLanguage::try_from(language_str.to_string()).unwrap()
    };
    
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

    jobs::init(project_location, project_name, project_language);
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
        let type_str = matches.get_one::<String>("type").unwrap();
        // Safe to unwrap as we already checked for valid enum strings
        project::UnitType::try_from(type_str.to_string()).unwrap()
    };

    jobs::add_unit(project_location, unit_name, unit_type.clone(), unit_path);
}