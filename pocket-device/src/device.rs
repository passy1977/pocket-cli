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
            Add => ret += "ADD_DEVICE",
            Mod => ret += "MOD_DEVICE",
            Rm => ret += "RM_DEVICE",
            Get => ret += "GET_DEVICE",
        }
        ret += DIVISOR;

        match &self.cmd {
            Rm | Get | Mod => {
                ret.push_str(&self.email);
                ret += DIVISOR;
                if let Some(uuid) = &self.uuid {
                    ret.push_str(uuid);    
                }
                ret += DIVISOR;
                if let Some(note) = &self.uuid {
                    ret.push_str(note);
                }
            }
            Add => ret.push_str(&self.email)
        };

        ret
    }
}