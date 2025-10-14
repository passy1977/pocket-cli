use crate::models::commands::{CliCommands, CliCommands::*, CliOptions, CliOptions::*};

#[derive(Debug)]
pub struct ParsedArgs {
    pub command: Option<CliCommands>,
    pub options: Vec<CliOptions>,
}

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
        "-P" | "--server-passwd" => Some(ServerPassword("".to_string())),
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
    // Prima unisci gli argomenti che sono racchiusi tra virgolette
    let processed_args = merge_quoted_args(args);
    let mut command: Option<CliCommands> = None;

    for arg in &processed_args {
        if command.is_none() {
            command = check_command(&arg);
        }
        
        if command.is_some() {
            return command;
        }
    }
    
    None
}

pub fn parse_full(args: &Vec<String>) -> ParsedArgs {
    // Prima unisci gli argomenti che sono racchiusi tra virgolette
    let processed_args = merge_quoted_args(args);
    let mut command: Option<CliCommands> = None;
    let mut options: Vec<CliOptions> = Vec::new();
    let mut i = 0;

    while i < processed_args.len() {
        let arg = &processed_args[i];
        
        // Controlla se è un comando
        if command.is_none() {
            if let Some(cmd) = check_command(&arg) {
                command = Some(cmd);
                i += 1;
                continue;
            }
        }
        
        // Controlla se è un'opzione
        if let Some(mut option) = check_option(&arg) {
            // Se c'è un argomento successivo, usalo come valore
            if i + 1 < processed_args.len() {
                let next_arg = &processed_args[i + 1];
                // Se il prossimo argomento non è un'altra opzione o comando
                if !next_arg.starts_with('-') && check_command(next_arg).is_none() {
                    option = match option {
                        ServerPassword(_) => ServerPassword(next_arg.clone()),
                        Email(_) => Email(next_arg.clone()),
                        Passwd(_) => Passwd(next_arg.clone()),
                        Name(_) => Name(next_arg.clone()),
                        Note(_) => Note(next_arg.clone()),
                        UUID(_) => UUID(next_arg.clone()),
                        Help(_) => Help(next_arg.clone()),
                    };
                    i += 1; // Salta il valore nel prossimo ciclo
                }
            }
            options.push(option);
        }
        
        i += 1;
    }
    
    ParsedArgs { command, options }
}

fn merge_quoted_args(args: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;
    
    while i < args.len() {
        let arg = &args[i];
        
        // Se l'argomento inizia con una virgoletta
        if arg.starts_with('"') {
            // Se inizia e finisce con virgolette nello stesso argomento
            if arg.len() > 1 && arg.ends_with('"') {
                let merged = arg[1..arg.len()-1].to_string();
                result.push(merged);
            } else {
                // Rimuovi la virgoletta iniziale
                let mut merged = arg[1..].to_string();
                i += 1;
                
                // Continua fino a trovare la virgoletta di chiusura
                while i < args.len() {
                    let current = &args[i];
                    if current.ends_with('"') {
                        // Aggiungi tutto tranne l'ultima virgoletta
                        if !merged.is_empty() {
                            merged.push(' ');
                        }
                        merged.push_str(&current[..current.len()-1]);
                        break;
                    } else {
                        // Aggiungi l'argomento intero
                        if !merged.is_empty() {
                            merged.push(' ');
                        }
                        merged.push_str(current);
                    }
                    i += 1;
                }
                
                result.push(merged);
            }
        } else {
            result.push(arg.clone());
        }
        
        i += 1;
    }
    
    result
}