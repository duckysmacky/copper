use clap::ArgMatches;
use crate::jobs;

pub mod command;

pub fn match_args(matches: ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("init") {
        jobs::handle_init(matches);
    }

    if let Some(matches) = matches.subcommand_matches("build") {
        jobs::handle_build(matches);
    }

    if let Some(matches) = matches.subcommand_matches("new") {
        jobs::handle_new(matches);
    }
}
