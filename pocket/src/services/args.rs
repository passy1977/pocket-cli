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
    let mut i = 0usize;
    while i < args.len() {
        let arg = &args[i];

        // If it's a command -> return immediately
        if let Some(cmd) = check_command(arg) {
            return Some(cmd);
        }

        // If it's an option (flag), also skip its value if present
        if arg.starts_with('-') {
            if i + 1 < args.len() {
                let next = &args[i + 1];
                // If the next one is not another flag, consider it a value and skip both
                if !next.starts_with('-') {
                    i += 2;
                    continue;
                }
            }
        }

        i += 1;
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_finds_command_get() {
        let args = vec![
            "get".to_string(),
        ];
        assert_eq!(parse(&args), Some(CliCommands::Get));
    }

    #[test]
    fn parse_finds_command_among_flags() {
        let args = vec![
            "-n".to_string(),
            "Antonio".to_string(),
            "get".to_string(),
        ];
        assert_eq!(parse(&args), Some(CliCommands::Get));
    }

    #[test]
    fn parse_finds_command_with_multitoken_value() {
        let args = vec![
            "-n".to_string(),
            "Antonio".to_string(),
            "Salsi".to_string(),
            "get".to_string(),
            "-e".to_string(),
            "test@test.com".to_string(),
        ];
        assert_eq!(parse(&args), Some(CliCommands::Get));
    }

    #[test]
    fn parse_returns_none_when_no_command() {
        let args = vec![
            "-n".to_string(),
            "Antonio".to_string(),
            "-e".to_string(),
            "test@test.com".to_string(),
        ];
        assert_eq!(parse(&args), None);
    }

    #[test]
    fn check_option_recognizes_short_flags() {
        assert!(matches!(check_option(&"-n".to_string()), Some(Name(_))));
        assert!(matches!(check_option(&"-e".to_string()), Some(Email(_))));
        assert!(matches!(check_option(&"-u".to_string()), Some(UUID(_))));
        assert!(matches!(check_option(&"-P".to_string()), Some(ServerPassword(_))));
    }

    #[test]
    fn check_option_recognizes_long_flags() {
        assert!(matches!(check_option(&"--name".to_string()), Some(Name(_))));
        assert!(matches!(check_option(&"--email".to_string()), Some(Email(_))));
        assert!(matches!(check_option(&"--uuid".to_string()), Some(UUID(_))));
        assert!(matches!(check_option(&"--server-passwd".to_string()), Some(ServerPassword(_))));
    }

    #[test]
    fn check_option_returns_none_for_unknown() {
        assert_eq!(check_option(&"--unknown".to_string()), None);
        assert_eq!(check_option(&"get".to_string()), None);
    }
}
