use std::path::PathBuf;
use clap::Args;
use crate::config::{ProjectLanguage, ProjectCompiler};

#[derive(Args)]
pub struct InitCommand {
    /// Specify the directory in which to create the Copper project in
    #[arg(
        default_value = "."
    )]
    pub location: PathBuf,
    
    /// Specify the language of the project
    #[arg(
        long = "lang", short,
        default_value = "c++",
        value_parser = ProjectLanguage::str_variants()
    )]
    pub language: String,
    
    /// Specify project's compiler
    #[arg(
        long, short,
        default_value = "g++",
        value_parser = ProjectCompiler::str_variants()
    )]
    pub compiler: String,
    
    /// Specify the project name
    /// 
    /// If not specified, the name will be derived from the directory name
    #[arg(
        long, short
    )]
    pub name: Option<String>,
    
    /// Generate an example project configuration
    #[arg(
        long,
        default_value = "true",
        conflicts_with = "minimal"
    )]
    pub example: bool,
    
    /// Generate a minimal project configuration
    #[arg(
        long,
        conflicts_with = "example"
    )]
    pub minimal: bool,
}