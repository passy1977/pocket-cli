use crate::models::commands::{CliCommands, CliCommands::Get};
use crate::models::model::Model;

#[derive(Clone)]
pub struct User {
    pub cmd: CliCommands,
    pub email: String,
    pub passwd: Option<String>,
    pub name: Option<String>
}

impl User {
    pub fn new() -> Self {
        User {
            cmd: Get,
            email: "".to_string(),
            passwd: None,
            name: None,
        }

    }
}

impl Model for User {
    fn cmd(&self) -> CliCommands {
        self.cmd.clone()
    }
}