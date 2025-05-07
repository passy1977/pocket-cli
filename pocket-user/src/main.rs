mod cli;
mod user;

use std::{env, fs, path};
use std::io::ErrorKind;
use std::process::exit;
use cli::check_args;
use pocket::fs::DATA_FOLDER;
use pocket::models::commands::CliOptions::*;
use pocket::Pocket;
use pocket::services::args::get_menu;
use crate::cli::parse;
use crate::user::User;

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
    
    let args: Vec<String> = vec!["add".to_string(),
                                 "-s".to_string(), "123456789".to_string(),
                                 "--email".to_string(), "passy.linux@zresa.it".to_string(),
                                 "-n".to_string(), "Passy".to_string(),
                                 "-p".to_string(), "qwerty".to_string(),
                                 "--note".to_string(), "note di note alla seconda".to_string(),
                                 "-u".to_string(), "2ff2fafd-6511-4236-91fb-a255c9696e9d".to_string(),
                                 "-s".to_string(), "12345678123456781234567812345678".to_string(),
    ];


    let tuple =  pocket.parse(&args, parse);
    
    let command = match tuple.0 {
        Some(value) => value,
        None => {
            eprintln!("Command not found");
            exit(1);
        }
    };
    
    let options = match tuple.1  {
        Ok(value) => value,
        Err(error) => {
            if let pocket::utils::Error::Msg(msg) = error {
                eprintln!("Parsing error: {msg}");    
            } else {
                eprintln!("Unhandled error");
            }
            exit(1);
        }
    };
   

    if check_args(&command, &options) {
        
        if let Some(Help(_)) = options.get("Help") {
            println!("{}", get_menu());
            exit(0);
        }


        match &pocket.property {
            None => {
                if let Some(ServerPassword(passwd)) = options.get("ServerPassword") {
                    match pocket.login_server(Some(passwd.to_string())) {
                        Ok(_) => {}
                        Err(error) => {
                            eprintln!("Login error:{error}");
                            println!("{}", get_menu());
                            exit(1);
                        }
                    }
                }
            }
            Some(_) => {
                match pocket.login_server(None) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Error:{error}");
                        println!("{}", get_menu());
                        exit(1);
                    }
                }
            }
        }

        
        
        let mut user = User::new();
        
        user.cmd = command;
        
        user.email = if let Some(Email(email)) = options.get("Email") {
            email.clone()
        } else {
            eprintln!("Email it's mandatory");
            println!("{}", get_menu());
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
    } else {
        eprintln!("Not logged on server and no passwd find");
        println!("{}", get_menu());
        exit(1);
    }
}
