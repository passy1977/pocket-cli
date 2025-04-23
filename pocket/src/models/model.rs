use crate::models::commands::CliCommands;

pub trait Model {
    fn cmd(&self) -> CliCommands;
}