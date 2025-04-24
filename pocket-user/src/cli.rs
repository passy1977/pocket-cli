use pocket::models::commands::{CliCommands, CliOptions};
use pocket::utils::{Error, Result};

pub fn check_args(args: &(Option<CliCommands>, Vec<CliOptions>)) -> Result<(CliCommands, Vec<CliCommands>)> {
    
    
    let (command, options) = args;
    
    if command.is_none() {
        return Err("Command not set")
    }
    
    if options.is_empty() {
        return Err("Options empty")
    }
    
    for  command in options {
        
    }

    Err("")
}



