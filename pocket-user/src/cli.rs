use clap::{Parser};
use pocket::models::commands::{Commands, Commands::*};
use pocket::models::user::User;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Server passwd
    #[arg(short, long)]
    server_passwd: Option<String>,

    #[command(subcommand)]
    cmd: Commands,

    /// User email
    #[arg(short, long)]
    email: String,

    /// User password
    #[arg(short, long)]
    passwd: Option<String>,

    /// User name
    #[arg(short, long)]
    name: Option<String>,

}


impl Cli {
    pub fn perform() -> (Option<String>, Option<User>) {

        let cli = Cli::parse();

        let mut server_passwd = Option::None;
        let mut user = User::new();

        if let Some(pwd) = cli.server_passwd.as_deref() {
            server_passwd = Option::Some(pwd.to_string());
        }

        // if let Some(user_cmd) = cli.user_cmd.as_deref() {
        //     user.cmd = match user_cmd {
        //         "ADD_USER" => Add,
        //         "MOD_USER" => Mod,
        //         "RM_USER" => Rm,
        //         "RM_GET" => Get,
        //         _ => None
        //     }
        // }



        user.email = cli.email.to_string();
        user.passwd = Some(cli.passwd.to_string());


        if let Some(user_name) = cli.name.as_deref() {
            user.name = Some(user_name.to_string());
        }

        match (&server_passwd, &user.cmd) {
            (Some(_), Add | Mod | Rm | Get) => (server_passwd, Some(user)),
            (_, _) => (Option::None, Option::None)
        }
    }
}