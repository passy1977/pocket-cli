use std::collections::HashMap;
use pocket::models::commands::{CliCommands, CliCommands::*, CliOptions, CliOptions::*};
use pocket::services::args::check_option;
use pocket::utils::{Error, Result};


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
                    UUID(_) => options.insert("UUID", UUID(arg.clone())),
                    Help(_) => options.insert("Help", Help(arg.clone()))
                };
                flag = None;
            }
        }
    }
    
    Ok(options)
}


pub fn check_args(command: &CliCommands, options: &HashMap<&'static str, CliOptions>) -> bool {
    match command {
        Add | Mod => options.get("Email").unwrap().is_empty() 
                            && options.get("Passwd").unwrap().is_empty()
                            && options.get("Name").unwrap().is_empty(),
        Rm | Get => options.get("Email").unwrap().is_empty()
    }
}

 
