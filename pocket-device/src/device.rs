use pocket::cli::DIVISOR;
use pocket::models::commands::{CliCommands, CliCommands::*};
use pocket::traits::command_to_server::StringToServer;

#[derive(Clone)]
pub struct Device {
    pub cmd: CliCommands,
    pub email: String,
    pub uuid: Option<String>,
    pub note: Option<String>
}

impl Device {
    pub fn new(email: String) -> Self {
        Device {
            cmd: Get,
            email,
            uuid: None,
            note: None
        }
    }
}

impl StringToServer for Device {
    fn get_string_to_sever(&self) -> String {
        let mut ret = "".to_string();

        match &self.cmd {
            Add => ret += "ADD_USER",
            Mod => ret += "MOD_USER",
            Rm => ret += "RM_USER",
            Get => ret += "GET_USER",
        }
        ret += DIVISOR;

        match &self.cmd {
            Add | Mod => {
                ret.push_str(&self.email);
                ret += DIVISOR;
                ret.push_str(&self.uuid.as_ref().unwrap());
                ret += DIVISOR;
                ret.push_str(&self.note.as_ref().unwrap());
            }
            Rm | Get => ret.push_str(&self.email)
        };

        ret
    }
}