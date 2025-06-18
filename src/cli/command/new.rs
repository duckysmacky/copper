use std::path::PathBuf;
use clap::{Args, Subcommand};
use crate::config::UnitType;

/// Add a new component to the Copper project
#[derive(Args)]
pub struct NewCommand {
    #[command(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Unit(UnitSubcommand)
}

#[derive(Args)]
pub struct UnitSubcommand {
    /// Specify the source directory of the unit
    #[arg(
        required = true
    )]
    pub source: PathBuf,
    
    /// Specify the type of the unit
    #[arg(
        required = true,
        value_parser = UnitType::str_variants()
    )]
    pub r#type: String,
}