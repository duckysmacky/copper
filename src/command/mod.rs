use clap::{command, Arg, ArgAction, ArgMatches, Command};
use crate::config::{project, unit};

mod handlers;

pub fn get_command() -> Command {
    command!()
        .subcommand(Command::new("init")
            .about("Initiate a new Copper project in the current directory")
            .arg(Arg::new("location")
                .help("Specify the directory in which to create the Copper project in")
                .default_value(".")
            )
            .arg(Arg::new("language")
                .help("Specify the language of the project")
                .long("lang")
                .default_value("c")
                .value_parser(project::ProjectLanguage::get_strings())
            )
            .arg(Arg::new("name")
                .help("Specify the project name")
                .long("name")
                .short('n')
            )
            .arg(Arg::new("example")
                .help("Generate an example project configuration")
                .long("example")
                .default_value("true")
                .action(ArgAction::SetTrue)
                .conflicts_with("minimal")
            )
            .arg(Arg::new("minimal")
                .help("Generate a minimal project configuration")
                .long("minimal")
                .action(ArgAction::SetTrue)
                .conflicts_with("example")
            )
        )
        .subcommand(Command::new("build")
            .about("Build the local Copper project")
            .arg(Arg::new("units")
                .help("Specify units to build")
                .action(ArgAction::Append)
            )
            .arg(Arg::new("location")
                .help("Specify the directory where the Copper project is located")
                .long("path")
                .default_value(".")
                .action(ArgAction::Set)
            )
        )
        .subcommand(Command::new("unit")
            .about("Add a new unit to the Copper project")
            .arg(Arg::new("source")
                .help("Specify the directory of the unit")
                .required(true)
            )
            .arg(Arg::new("type")
                .help("Specify the type of the unit")
                .required(true)
                .value_parser(unit::UnitType::get_strings())
            )
            .arg(Arg::new("location")
                .help("Specify the directory where the Copper project is located")
                .required(false)
                .default_value(".")
            )
        )
}

pub fn match_args(matches: ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("init") {
        handlers::handle_init(matches);
    }

    if let Some(matches) = matches.subcommand_matches("build") {
        handlers::handle_build(matches);
    }

    if let Some(matches) = matches.subcommand_matches("unit") {
        handlers::handle_unit(matches);
    }
}