use clap::ArgMatches;

pub mod command;
mod handlers;

pub fn match_args(matches: ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("init") {
        handlers::handle_init(matches);
    }

    if let Some(matches) = matches.subcommand_matches("build") {
        handlers::handle_build(matches);
    }

    if let Some(matches) = matches.subcommand_matches("new") {
        handlers::handle_new(matches);
    }
}