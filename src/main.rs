mod commands;
mod project;
mod file;

use clap::{command, Arg, Command};

fn main() {
    let matches = command!()
        .subcommand(Command::new("init")
            .about("Initiate a new Copper project in the current directory")
            .arg(Arg::new("directory")
                .help("Specify the directory in which to create the Copper project in")
                .required(false)
            )
            .arg(Arg::new("name")
                .help("Specify the project name")
                .short('n')
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        commands::handle_init(matches);
    }
}
