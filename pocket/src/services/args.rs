use crate::models::commands::{CliCommands, CliCommands::*};

fn check_command(arg: &String) -> Option<CliCommands> {
    match arg.as_str() {
        "add" => Some(Add),
        "mod" => Some(Mod),
        "rm" => Some(Rm),
        "get" => Some(Get),
        _ => None
    }
}

pub fn parse(args: &Vec<String>) -> Option<CliCommands> {
    
    let mut command : Option<CliCommands> = None;

    for arg in args {
        if command.is_none() {
            command = check_command(&arg);
            return command
        }
    }
    
    None
}