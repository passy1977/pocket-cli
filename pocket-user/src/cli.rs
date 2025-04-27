use std::collections::HashMap;
use pocket::models::commands::{CliCommands, CliOptions, CliOptions::*};
use pocket::utils::{Error, Result};

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

pub fn parse(args: &Vec<String>) -> Result<HashMap<&'static str, CliOptions>, Error> {
    
    let mut options : HashMap<&'static str, CliOptions> = HashMap::new();

    let mut flag : Option<CliOptions> = None;
    for arg in args {
        match &flag {
            None => flag = check_option(&arg),
            Some(option) => {
                match option {
                    ServerPassword(_) => options.insert("ServerPassword", ServerPassword(arg.clone())),
                    Email(_) => options.insert("Email", Email(arg.clone())),
                    Passwd(_) => options.insert("Passwd", Passwd(arg.clone())),
                    Name(_) => options.insert("Name", Name(arg.clone())),
                    Note(_) => options.insert("Note", Note(arg.clone())),
                    UUID(_) => options.insert("UUID", Note(arg.clone()))
                };
                flag = None;
            }
        }
    }
    
    Ok(options)
}


pub fn check_args(commands: &CliCommands, options: &HashMap<&'static str, CliOptions>) -> Result<()> {
    
    // for (_, mut option) in options_opt.into_iter().enumerate() {
    //     match option {
    //         ServerPassword(s) => {
    //             options.insert("ServerPassword", ServerPassword(s.clone()));
    //         }
    //         Email(s) => {
    //             options.insert("Email", Email(s.clone()));
    //         }
    //         Passwd(s) => {
    //             options.insert("Passwd", Passwd(s.clone()));
    //         }
    //         Name(s) => {
    //             options.insert("Name", Name(s.clone()));
    //         }
    //         Note(s) => {
    //             options.insert("Note", Note(s.clone()));
    //         }
    //         UUID(s) => {
    //             options.insert("UUID", Note(s.clone()));
    //         }
    //     }
    // }

    // let command = if let Some(cmd) = command_opt {
    //   cmd.clone()
    // } else {
    //   return Err("Command not set")
    // };

    Ok(())
}



