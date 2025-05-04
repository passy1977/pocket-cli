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

pub fn check_option(arg: &String) -> Option<CliOptions> {
    match arg.as_str() {
        "-s" | "--server-passwd" => Some(ServerPassword("".to_string())),
        "-e" | "--email" => Some(Email("".to_string())),
        "-p" | "--passwd" => Some(Passwd("".to_string())),
        "-n" | "--name" => Some(Name("".to_string())),
        "--note" => Some(Note("".to_string())),
        "-u" | "--uuid" => Some(UUID("".to_string())),
        "-h" | "--help" => Some(Help("".to_string())),
        _ => None
    }
}

pub(crate) fn parse(args: &Vec<String>) -> Option<CliCommands> {
    
    let mut command : Option<CliCommands> = None;

    for arg in args {
        if command.is_none() {
            command = check_command(&arg);
            return command
        }
    }
    
    None
}

pub fn get_menu() -> String {
    "[Menu]".to_string()
}