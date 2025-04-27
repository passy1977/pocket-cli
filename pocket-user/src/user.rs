use pocket::models::commands::{CliCommands, CliCommands::Get};
use pocket::traits::command_to_server::StringToServer;

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

impl StringToServer for User {
    fn get_string_to_sever(&self) -> String {
        "".to_string()
    }
}