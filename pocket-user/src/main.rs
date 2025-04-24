mod cli;

use std::{env, fs, path};
use std::io::ErrorKind;
use std::process::exit;
use cli::check_args;
use pocket::constants::fs::DATA_FOLDER;
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

    let pocket = Pocket::new(base_path);
    
    let (passwd_opt, user_opt) = check_args(&pocket.parse());
    
    if !pocket.logged {
        if let Some(passwd) = passwd_opt {
            match pocket.login_server(passwd) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("Server passwd mismatch:{error}");
                    exit(1);
                }
            }
        }
    }

    let user;
    if let Some(usr) = user_opt {
        user = usr;
        match pocket.execute(user) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("{error}");
                exit(1);
            }
        }
    } else {
        eprintln!("User parameter missing");
        exit(1);
    }

}
