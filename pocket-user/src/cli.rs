use clap::Parser;
use pocket::models::commands::{Commands, Commands::*};
use pocket::models::user::User;
use pocket::Pocket;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Server passwd
    #[arg(short)]
    server_passwd: Option<String>,

    #[command(subcommand)]
    cmd: Commands,

    /// User email
    #[arg(short)]
    email: String,

    /// User password
    #[arg(short)]
    passwd: Option<String>,

    /// User name
    #[arg(short)]
    name: Option<String>,
}

impl Cli {
    pub fn perform() -> (Option<String>, Option<User>) {

        let cli = Cli::parse();

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


