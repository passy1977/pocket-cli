use std::env;
use crate::models::commands::{CliCommands, CliCommands::*, CliOptions, CliOptions::*};

fn check_command(options: &mut Vec<CliCommands>, arg: &String) {
    match arg.as_str() {
        "add" => options.push(Add),
        "mod" => options.push(Mod),
        "rm" => options.push(Rm),
        "get" => options.push(Get),
        _ => {}
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

pub fn parse() -> (Vec<CliCommands>, Vec<CliOptions>) {
    let args: Vec<String> = env::args().collect();

    let mut commands : Vec<CliCommands> = Vec::new();
    let mut options : Vec<CliOptions> = Vec::new();
    
    let mut flag : Option<CliOptions> = None;
    for arg in args {
        check_command(&mut commands, &arg);

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
            }
        }
    }
    
    (commands, options)
}