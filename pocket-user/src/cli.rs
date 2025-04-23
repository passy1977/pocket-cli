use std::env;
use clap::Parser;
use pocket::models::commands::{CliCommands, CliCommands::*};
use pocket::models::user::User;

pub struct Cli {
    /// Server passwd
    server_passwd: Option<String>,
    
    cmd: CliCommands,

    /// User email
    email: String,

    /// User password
    passwd: Option<String>,

    /// User name
    name: Option<String>,
}

impl Cli {
    pub fn perform() -> (Option<String>, Option<User>) {

        let args: Vec<String> = env::args().collect();
        
        let cli = Cli{
            server_passwd: None,
            cmd: CliCommands::Add,
            email: "".to_string(),
            passwd: None,
            name: None,
        };

        let mut server_passwd = None;
        let mut user = User::new();

        if let Some(pwd) = cli.server_passwd.as_deref() {
            server_passwd = Some(pwd.to_string());
        }

        user.cmd = cli.cmd;

        user.email = cli.email.to_string();
        user.passwd = cli.passwd;

        if let Some(user_name) = cli.name.as_deref() {
            user.name = Some(user_name.to_string());
        }

        match (&server_passwd, &user.cmd) {
            (Some(_), Add | Mod | Rm | Get) => (server_passwd, Some(user)),
            (_, _) => (None, None)
        }
    }
}


