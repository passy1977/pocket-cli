use pocket::models::commands::{CliCommands, CliCommands::Get};
use pocket::traits::command_to_server::StringToServer;

#[derive(Clone)]
pub struct Device {
    pub cmd: CliCommands,
    pub email: String,
    pub uuid: String,
    pub note: Option<String>
}

impl Device {
    pub fn new(email: String) -> Self {
        Device {
            cmd: Get,
            email,
            uuid: "".to_string(),
            note: None
        }
    }
}

impl StringToServer for Device {
    fn get_string_to_sever(&self) -> String {
        "".to_string()
    }
}