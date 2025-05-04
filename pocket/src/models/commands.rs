use CliOptions::*;

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
    UUID(String),
    Help(String),
}


impl CliOptions {
    pub fn value(&self) -> String {
        match self {
            ServerPassword(s) => s.clone(),
            Email(s) => s.clone(),
            Passwd(s) => s.clone(),
            Name(s) => s.clone(),
            Note(s) => s.clone(),
            UUID(s) => s.clone(),
            Help(s) => s.clone()
        }
    }
    
    pub fn is_empty(&self) -> bool {
        match self {
            ServerPassword(s) => s.is_empty(),
            Email(s) => s.is_empty(),
            Passwd(s) => s.is_empty(),
            Name(s) => s.is_empty(),
            Note(s) => s.is_empty(),
            UUID(s) => s.is_empty(),
            Help(s) => s.is_empty()
        }
    }
}
