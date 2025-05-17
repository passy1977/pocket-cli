use std::{env, fs, path};
use std::io::ErrorKind;
use std::process::exit;
use pocket::fs::DATA_FOLDER;
use pocket::models::commands::CliOptions::{Email, Help, Note, ServerPassword, UUID};
use pocket::Pocket;
use crate::cli::{check_args, get_args, get_menu, parse};
use crate::device::Device;

mod cli;
mod device;

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

    let tuple =  pocket.parse(&get_args(), parse);


    let command = match tuple.0 {
        Some(value) => value,
        None => {
            eprintln!("Command not found");
            println!("{}", get_menu());
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

                if !options.contains_key("ServerPassword") {
                    eprintln!("Server password not found");
                    println!("{}", get_menu());
                    exit(1);
                }

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
        
        let mut user = Device::new(if let Some(Email(email)) = options.get("Email") {
            email.clone()
        } else {
            eprintln!("Email it's mandatory");
            println!("{}", get_menu());
            exit(1);
        });

        user.cmd = command;
        
        user.uuid = if let Some(UUID(passwd)) = options.get("UUID") {
            Some(passwd.clone())
        } else {
            None
        };

        user.note = if let Some(Note(name)) = options.get("Note") {
            Some(name.clone())
        } else {
            None
        };

        match pocket.execute(user) {
            Ok(ret) => {
                eprintln!("{}", ret.trim());
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