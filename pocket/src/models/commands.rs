use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    None,
    Add,
    Mod,
    Rm,
    Get
}