use clap::{CommandFactory, Parser};

mod file;
mod compiler;
mod error;
mod cli;
mod config;
mod jobs;

fn main() {
    let cli_command = cli::command::Cli::command();
    
    cli::match_args(cli_command.get_matches());
}
