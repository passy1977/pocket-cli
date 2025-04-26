mod cli;

use std::{env, fs, path};
use std::io::ErrorKind;
use std::process::exit;
use cli::check_args;
use pocket::constants::fs::DATA_FOLDER;
use pocket::models::commands::{CliOptions::*};
use pocket::models::user::User;
use pocket::Pocket;

fn main() {
    let mut base_path = match env::var("HOME") {
        Ok(home) => home,
        Err(e) => {
            eprintln!("Var $HONE not defined: {}", e);
            exit(1);
        }
    };

    base_path.push(path::MAIN_SEPARATOR);
    base_path.push_str(DATA_FOLDER);

    match fs::metadata(&base_path) {
        Ok(_) => {},
        Err(e) if e.kind() == ErrorKind::NotFound => {
            fs::create_dir(&base_path).expect("Impossible to create base dir");
        },
        Err(e) => eprintln!("Error: {:?}", e),
    }

    let mut pocket = Pocket::new(base_path);
    
    if let Ok( (command, options) ) = check_args(&pocket.parse()) {
        
        if !pocket.logged {
            if let Some(ServerPassword(passwd)) = options.get("ServerPassword") {
                match pocket.login_server(passwd.to_string()) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Server passwd mismatch:{error}");
                        exit(1);
                    }
                }
            }
        } else {
            eprintln!("Not logged on server and no passwd find");
            exit(1);
        }

        let mut user = User::new();
        
        user.cmd = command;
        
        user.email = if let Some(Email(email)) = options.get("Email") {
            email.clone()
        } else {
            eprintln!("Email it's mandatory");
            exit(1);
        };

        user.passwd = if let Some(Passwd(passwd)) = options.get("Passwd") {
            Some(passwd.clone())
        } else {
            None
        };

        user.name = if let Some(Name(name)) = options.get("Name") {
            Some(name.clone())
        } else {
            None
        };
        
        match pocket.execute(user) {
            Ok(ret) => {
                eprintln!("{ret}");
                exit(0);
            }
            Err(error) => {
                eprintln!("{error}");
                exit(1);
            }
        }
    }
}
