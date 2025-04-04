
use crate::models::user::User;
use crate::models::commands::{Commands, Commands::Get};

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