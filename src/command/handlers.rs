use std::env;
use std::path::{Path, PathBuf};
use clap::ArgMatches;
use crate::jobs;
use crate::config::{project, unit};

pub fn handle_init(matches: &ArgMatches) {
    let project_language = {
        let language_str = matches.get_one::<String>("language").unwrap();
        // Safe to unwrap as we already checked for valid enum strings
        project::ProjectLanguage::try_from(language_str.to_string()).unwrap()
    };
    
    let project_location = {
        let location = matches.get_one::<String>("location").unwrap();
        Path::new(location)
    };

    let project_name = match matches.get_one::<String>("name") {
        Some(name) => String::from(name),
        None => {
            let directory = if project_location == Path::new("../..") {
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

    let generate_example = matches.get_flag("example") && !matches.get_flag("minimal");

    jobs::init(project_location, project_name, project_language, generate_example);
}

pub fn handle_build(matches: &ArgMatches) {
    let units = matches.get_many::<String>("units");
    
    let project_location = {
        let location = matches.get_one::<String>("location").unwrap();
        Path::new(location)
    };

    jobs::build(units, project_location);
}

pub fn handle_new(matches: &ArgMatches) {
    let project_location = {
        let location = matches.get_one::<String>("location").unwrap();
        Path::new(location)
    };
 
    if let Some(matches) = matches.subcommand_matches("unit") {
        let unit_source = matches.get_one::<String>("source").unwrap();
        let unit_path = PathBuf::from(unit_source);
        let unit_name = Path::new(unit_source).file_name().unwrap().to_str().unwrap();
        let unit_type = {
            let type_str = matches.get_one::<String>("type").unwrap();
            // Safe to unwrap as we already checked for valid enum strings
            unit::UnitType::try_from(type_str.to_string()).unwrap()
        };

        jobs::new::new_unit(project_location, unit_name, unit_type.clone(), unit_path);
    }
}