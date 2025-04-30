use pocket::cli::DIVISOR;
use pocket::models::commands::{CliCommands, CliCommands::Get};
use pocket::models::commands::CliCommands::{Add, Mod, Rm};
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
                ret.push_str(&self.passwd.as_ref().unwrap());
                ret += DIVISOR;
                ret.push_str(&self.name.as_ref().unwrap());
            }
            Rm | Get => ret.push_str(&self.email)
        };

        ret
    }
}