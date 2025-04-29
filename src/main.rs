mod file;
mod compiler;
mod error;
mod command;
mod config;
mod jobs;

fn main() {
    let matches = command::get_command().get_matches();
    command::match_args(matches);
}
