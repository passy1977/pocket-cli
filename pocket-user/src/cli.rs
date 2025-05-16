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
            // "add".to_string(),
            //  "-s".to_string(), "123456789".to_string(),
            //  "--email".to_string(), "passy.linux@zresa.it".to_string(),
            //  "-n".to_string(), "Passy".to_string(),
            //  "-p".to_string(), "qwerty".to_string(),
            //  "--note".to_string(), "note di note alla seconda".to_string(),
            //  "-u".to_string(), "2ff2fafd-6511-4236-91fb-a255c9696e9d".to_string(),
            //  "-s".to_string(), "12345678123456781234567812345678".to_string(),
            "get".to_string(),
            "-e".to_string(),
            "antonio@salsi.it".to_string(),
            "-P".to_string(),
            "12345678123456781234567812345678".to_string(),

        ]
    }

    #[cfg(not(debug_assertions))]
    {
        let ret : Vec<String> = env::args().collect();

        // let mut ret : Vec<String> = Vec::new();
        // for arg in env::args() {
        //     ret.push(arg.clone())
        // }
        // 
        // for v in &ret {
        //     println!("--->{v}<---");
        // }
        // 
        ret
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
            let passwd = options.get("Passwd").is_some_and(|option: &CliOptions| {
                !option.is_empty()
            });
            let name = options.get("Name").is_some_and(|option: &CliOptions| {
                !option.is_empty()
            });
            email && passwd && name
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
    add                             add new user options mandatory: email, passwd, name  
    mod                             modify user options mandatory: email, passwd, name
    rm                              remove user options mandatory: email
    get                             get user information options mandatory: email

options:
    -P, --server-passwd <passwd>    set pocket server password, once the password is provided the system will remember it
    -e, --email <email>             set user email
    -p, --passwd <passwd>           set user passwd
    -n, --name <name>               set user name
    -h, --help <command>            show help
")
} 
