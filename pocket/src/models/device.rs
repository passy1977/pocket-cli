
use crate::models::user::User;
use crate::models::commands::{Commands, Commands::Get};
use crate::models::model::Model;

#[derive(Clone)]
pub struct Device {
    pub cmd: Commands,
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
            note: Option::None
        }
    }
}

impl Model for Device {
    fn cmd(&self) -> Commands {
        self.cmd.clone()
    }
}