use clap::Subcommand;
use crate::models::user::User;
use crate::models::device::DeviceCmd::Get;

#[derive(Clone)]
pub enum DeviceCmd {
    None,
    Add,
    Mod,
    Rm,
    Get
}

#[derive(Clone)]
pub struct Device {
    pub cmd: DeviceCmd,
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