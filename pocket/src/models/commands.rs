
#[derive(Debug, Clone)]
pub enum CliCommands {
    Add,
    Mod,
    Rm,
    Get
}

#[derive(Debug, Clone)]
pub enum CliOptions {
    ServerPassword(String),
    Email(String),
    Passwd(String),
    Name(String),
    Note(String),
    UUID(String)
}

