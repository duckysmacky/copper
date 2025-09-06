use std::path::PathBuf;
use clap::{Args, Subcommand};
use crate::project::UnitType;

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
        long, short,
        required = true
    )]
    pub source: PathBuf,
    
    /// Specify the type of the unit
    #[arg(
        long, short,
        required = true,
        value_parser = UnitType::str_variants()
    )]
    pub r#type: String,
    
    /// Specify the name of the unit
    /// 
    /// If not specified, the name will be derived from the unit's source
    /// directory name
    #[arg(
        long, short
    )]
    pub name: Option<String>,
}
