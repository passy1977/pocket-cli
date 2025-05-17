use std::collections::HashMap;
use std::env;
use std::path::Path;
use pocket::models::commands::{CliCommands, CliCommands::*, CliOptions, CliOptions::*};
use pocket::services::args::check_option;
use pocket::utils::{Error, Result};


pub fn get_args() -> Vec<String> {

    #[cfg(debug_assertions)]
    {
        vec![
            "get".to_string(),
            "-e".to_string(),
            "test@test.com".to_string(),
            "-P".to_string(),
            "12345678123456781234567812345678".to_string(),
        ]
    }

    #[cfg(not(debug_assertions))]
    {
        env::args().collect()
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
                    UUID(_) => options.insert("UUID", UUID(arg.clone())),
                    Help(_) => options.insert("Help", Help(arg.clone())),
                };
                flag = None;
            }
        }
    }

    Ok(options)
}


pub fn check_args(command: &CliCommands, options: &HashMap<&'static str, CliOptions>) -> bool {
    match command {
        Add | Mod => {
            let email = options.get("Email").is_some_and(|option: &CliOptions| {
                !option.is_empty()
            });
            let uuid = options.get("UUID").is_some_and(|option: &CliOptions| {
                !option.is_empty()
            });
            email && uuid
        }
        Rm | Get => options.get("Email").is_some_and(|option: &CliOptions| {
            !option.is_empty()
        })
    }
}

pub fn get_menu() -> String {
    let binary_name = match env::current_exe() {
        Ok(path) =>path.file_stem()
            .unwrap_or_else(|| Path::new("unknown").as_os_str())
            .to_str()
            .unwrap()
            .to_string(),
        Err(e) => e.to_string(),
    };

    format!(r"
usage: {binary_name} command [options]

commands:
    add                             add new device options mandatory: email  
    mod                             modify device options mandatory: email, UUID
    rm                              remove device options mandatory: email, UUID
    get                             get device information options mandatory: email, UUID

options:
    -P, --server-passwd <passwd>    set pocket server password, once the password is provided the system will remember it
    -u, --email <email>             set device email
    -p, --uuid <uuid>               set device uuid
    --note <note>                   set device note
    -h, --help <command>            show help
")
} 
