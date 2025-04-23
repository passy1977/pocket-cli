
use crate::models::user::User;
use crate::models::commands::{CliCommands, CliCommands::Get};
use crate::models::model::Model;

#[derive(Clone)]
pub struct Device {
    pub cmd: CliCommands,
    pub user: User,
    pub uuid: String,
    pub note: Option<String>
}

impl Device {
    pub fn new(user: User) -> Self {
        Device {
            cmd: Get,
            user,
            uuid: "".to_string(),
            note: None
        }
    }
}

impl Model for Device {
    fn cmd(&self) -> CliCommands {
        self.cmd.clone()
    }
}