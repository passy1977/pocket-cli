use std::env;
use crate::models::commands::{CliCommands, CliCommands::*, CliOptions, CliOptions::*};

fn check_command(arg: &String) -> Option<CliCommands> {
    match arg.as_str() {
        "add" => Some(Add),
        "mod" => Some(Mod),
        "rm" => Some(Rm),
        "get" => Some(Get),
        _ => None
    }
}

fn check_option(arg: &String) -> Option<CliOptions> {
    match arg.as_str() {
        "-s" | "--server-passwd" => Some(ServerPassword("".to_string())),
        "-e" | "--email" => Some(Email("".to_string())),
        "-p" | "--passwd" => Some(Passwd("".to_string())),
        "-n" | "--name" => Some(Name("".to_string())),
        "--note" => Some(Note("".to_string())),
        "-u" | "--uuid" => Some(UUID("".to_string())),
        _ => None
    }
}

pub fn parse() -> (Option<CliCommands>, Vec<CliOptions>) {
    //let args: Vec<String> = env::args().collect();

    let args: Vec<String> = vec!["-s".to_string(), "123456789".to_string(), "--email".to_string(), "123456789".to_string()];


    let mut command : Option<CliCommands> = None;
    let mut options : Vec<CliOptions> = Vec::new();
    
    let mut flag : Option<CliOptions> = None;
    for arg in args {
        if command.is_none() {
            command = check_command(&arg);
        }

        match &flag {
            None => flag = check_option(&arg),
            Some(option) => {
                match &option {
                    ServerPassword(_) => options.push(ServerPassword(arg)),
                    Email(_) => options.push(Email(arg)),
                    Passwd(_) => options.push(Passwd(arg)),
                    Name(_) => options.push(Name(arg)),
                    Note(_) => options.push(Note(arg)),
                    UUID(_) => options.push(UUID(arg)),
                }
                flag = None;
            }
        }
    }
    
    (command, options)
}