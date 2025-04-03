use crate::models::user::UserCmd::*;

#[derive(Clone)]
pub enum UserCmd {
    None,
    Add,
    Mod,
    Rm,
    Get
}


#[derive(Clone)]
pub struct User {
    pub cmd: UserCmd,
    pub email: String,
    pub passwd: Option<String>,
    pub name: Option<String>
}

impl User {
    pub fn new() -> Self {
        User {
            cmd: Get,
            email: "".to_string(),
            passwd: Option::None,
            name: Option::None,
        }

    }
}