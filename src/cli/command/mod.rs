mod init;
mod build;
mod new;

use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Specify the directory where the Copper project is located
    #[arg(
        long = "path",
        default_value = ".",
        global = true,
    )]
    pub location: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Copper project in the current directory
    Init(init::InitCommand),

    /// Build the local Copper project
    Build(build::BuildCommand),

    /// Create a new Copper project
    New(new::NewCommand),
}