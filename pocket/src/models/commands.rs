use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Add,
    Mod,
    Rm,
    Get
}