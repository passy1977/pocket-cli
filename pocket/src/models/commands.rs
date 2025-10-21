use std::fmt;
use std::fmt::{Display, Formatter};
use CliOptions::*;
use CliCommands::*;

#[derive(Debug, Clone, PartialEq)]
pub enum CliCommands {
    Add,
    Mod,
    Rm,
    Get
}

impl Display for CliCommands {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Add => write!(f, "Add"),
            Mod => write!(f, "Mod"),
            Rm => write!(f, "Rm"),
            Get => write!(f, "Get")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CliOptions {
    ServerPassword(String),
    Email(String),
    Passwd(String),
    Name(String),
    Note(String),
    UUID(String),
    Help(String),
}

impl Display for CliOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ServerPassword(s) => write!(f, "ServerPassword:{s}"),
            Email(s) => write!(f, "Email:{s}"),
            Passwd(s) => write!(f, "Passwd:{s}"),
            Name(s) => write!(f, "Name:{s}"),
            Note(s) => write!(f, "Note:{s}"),
            UUID(s) => write!(f, "UUID:{s}"),
            Help(s) => write!(f, "Help:{s}"),
        }
    }
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
