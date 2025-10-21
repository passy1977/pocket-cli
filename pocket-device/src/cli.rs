use std::collections::HashMap;
use std::env;
use std::path::Path;
use pocket::models::commands::{CliCommands, CliCommands::*, CliOptions, CliOptions::*};
use pocket::services::args::check_option;
use pocket::utils::{Error, Result};

pub fn get_args() -> Vec<String> {

    #[cfg(feature = "test_args")]
    {
        vec![
            "get".to_string(),
            "-u".to_string(),
            "44fd0bd8-46bd-465f-b49f-2a908defcb24".to_string(),
            "-e".to_string(),
            "test@test.com".to_string(),
            "-P".to_string(),
            "12345678123456781234567812345678".to_string(),
        ]
    }

    #[cfg(not(feature = "test_args"))]
    {
        #[cfg(debug_assertions)]
        for (index, arg) in env::args().enumerate() {
            println!("Argument {}: {}", index, arg);
        }
        env::args().collect()
    }

}

pub fn parse(args: &Vec<String>) -> Result<HashMap<&'static str, CliOptions>, Error> {

    let mut options : HashMap<&'static str, CliOptions> = HashMap::new();

    let mut flag : Option<CliOptions> = None;
    let mut i = 0usize;
    while i < args.len() {
        let arg = &args[i];

        match &flag {
            None => {
                flag = check_option(&arg);
                i += 1;
            }
            Some(option) => {
                // Collect value: the current token + all following tokens
                // that are not flags (don't start with '-') and are not known
                // commands (add, mod, rm, get). This supports both quoted strings
                // (the shell already passes them as a single token) and unquoted
                // multi-token values like: -n Antonio Salsi
                let mut value = arg.clone();
                i += 1;
                while i < args.len() {
                    let next = &args[i];
                    if next.starts_with('-') {
                        break;
                    }
                    match next.as_str() {
                        "add" | "mod" | "rm" | "get" => break,
                        _ => {
                            value.push(' ');
                            value.push_str(next);
                            i += 1;
                        }
                    }
                }

                match option {
                    ServerPassword(_) => { options.insert("ServerPassword", ServerPassword(value.clone())); },
                    Email(_) => { options.insert("Email", Email(value.clone())); },
                    Passwd(_) => { options.insert("Passwd", Passwd(value.clone())); },
                    Name(_) => { options.insert("Name", Name(value.clone())); },
                    Note(_) => { options.insert("Note", Note(value.clone())); },
                    UUID(_) => { options.insert("UUID", UUID(value.clone())); },
                    Help(_) => { options.insert("Help", Help(value.clone())); },
                };
                flag = None;
            }
        }
    }

    Ok(options)
}


pub fn check_args(command: &CliCommands, options: &HashMap<&'static str, CliOptions>) -> bool {
    match command {
        Rm | Get | Mod => {
            let email = options.get("Email").is_some_and(|option: &CliOptions| {
                !option.is_empty()
            });
            let uuid = options.get("UUID").is_some_and(|option: &CliOptions| {
                !option.is_empty()
            });
            email && uuid
        }
        Add => options.get("Email").is_some_and(|option: &CliOptions| {
            !option.is_empty()
        })
    }
}

pub fn get_menu(error_msg: Option<&str>) -> String {
    if let Some(error_msg) = error_msg {
        eprintln!("\x1b[31m{error_msg}\x1b[0m")
    }
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
    -e, --email <email>             set device email
    -u, --uuid <uuid>               set device uuid
    --note <note>                   set device note
    -h, --help <command>            show help
")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_token_values() {
        let args = vec![
            "-e".to_string(),
            "test@test.com".to_string(),
            "-u".to_string(),
            "44fd0bd8-46bd-465f-b49f-2a908defcb24".to_string(),
        ];
        let result = parse(&args).unwrap();

        assert!(result.contains_key("Email"));
        assert_eq!(result.get("Email").unwrap().value(), "test@test.com");
        assert!(result.contains_key("UUID"));
        assert_eq!(result.get("UUID").unwrap().value(), "44fd0bd8-46bd-465f-b49f-2a908defcb24");
    }

    #[test]
    fn parse_multitoken_name_value() {
        let args = vec![
            "-n".to_string(),
            "Antonio".to_string(),
            "Salsi".to_string(),
            "-e".to_string(),
            "test@test.com".to_string(),
        ];
        let result = parse(&args).unwrap();

        assert!(result.contains_key("Name"));
        assert_eq!(result.get("Name").unwrap().value(), "Antonio Salsi");
        assert!(result.contains_key("Email"));
        assert_eq!(result.get("Email").unwrap().value(), "test@test.com");
    }

    #[test]
    fn parse_quoted_string_as_single_token() {
        // Simula come la shell passa una stringa quotata: come singolo token
        let args = vec![
            "-n".to_string(),
            "Antonio Salsi".to_string(),
            "-e".to_string(),
            "test@test.com".to_string(),
        ];
        let result = parse(&args).unwrap();

        assert!(result.contains_key("Name"));
        assert_eq!(result.get("Name").unwrap().value(), "Antonio Salsi");
    }

    #[test]
    fn parse_stops_at_next_command() {
        let args = vec![
            "-n".to_string(),
            "Antonio".to_string(),
            "get".to_string(),
            "-e".to_string(),
            "test@test.com".to_string(),
        ];
        let result = parse(&args).unwrap();

        // Il valore di -n deve essere solo "Antonio", non "Antonio get"
        assert!(result.contains_key("Name"));
        assert_eq!(result.get("Name").unwrap().value(), "Antonio");
        assert!(result.contains_key("Email"));
    }

    #[test]
    fn check_args_add_requires_email() {
        let mut options = HashMap::new();
        options.insert("Email", Email("test@test.com".to_string()));

        assert!(check_args(&Add, &options));

        let empty_options = HashMap::new();
        assert!(!check_args(&Add, &empty_options));
    }

    #[test]
    fn check_args_get_requires_email_and_uuid() {
        let mut options = HashMap::new();
        options.insert("Email", Email("test@test.com".to_string()));
        options.insert("UUID", UUID("44fd0bd8-46bd-465f-b49f-2a908defcb24".to_string()));

        assert!(check_args(&Get, &options));

        // Solo email non basta
        let mut incomplete = HashMap::new();
        incomplete.insert("Email", Email("test@test.com".to_string()));
        assert!(!check_args(&Get, &incomplete));
    }
}

