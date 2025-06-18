mod file;
mod compiler;
mod error;
mod cli;
mod config;
mod jobs;

fn main() {
    let matches = cli::get_command().get_matches();
    cli::match_args(matches);
}
