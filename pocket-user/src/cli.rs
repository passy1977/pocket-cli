use std::collections::HashMap;
use pocket::models::commands::{CliCommands, CliOptions, CliOptions::*};
use pocket::utils::Result;

pub fn check_args(args: &(Option<CliCommands>, Vec<CliOptions>)) -> Result<(CliCommands, HashMap<&'static str, CliOptions>)> {


    let (command_opt, options_opt) = args;

    if command_opt.is_none() {
        return Err("Command not set")
    }

    if options_opt.is_empty() {
        return Err("Options empty")
    }

    let mut options : HashMap<&'static str, CliOptions> = HashMap::new();

    for (i, option) in options_opt.into_iter().enumerate() {
        match option {
            ServerPassword(s) => {
                options.insert("ServerPassword", ServerPassword(s.clone()));
            }
            Email(s) => {
                options.insert("Email", Email(s.clone()));
            }
            Passwd(s) => {
                options.insert("Passwd", Passwd(s.clone()));
            }
            Name(s) => {
                options.insert("Name", Name(s.clone()));
            }
            Note(s) => {
                options.insert("Note", Note(s.clone()));
            }
            UUID(s) => {
                options.insert("UUID", Note(s.clone()));
            }
        }
    }

    let command = if let Some(cmd) = command_opt {
      cmd.clone()
    } else {
      return Err("Command not set")
    };

    Ok((command, options))
}



