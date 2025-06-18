use clap::Args;

#[derive(Args)]
pub struct BuildCommand {
    /// Specify the units to build
    #[arg(
        action = clap::ArgAction::Append,
    )]
    pub units: Option<Vec<String>>,
}