use crate::models::commands::{Commands, Commands::Get};


#[derive(Clone)]
pub struct User {
    pub cmd: Commands,
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