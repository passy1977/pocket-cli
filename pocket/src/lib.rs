pub mod models;
pub mod services;
pub mod utils;
pub mod traits;
mod database;

use crate::models::commands::{CliCommands, CliOptions};
use crate::models::property::Property;
use crate::traits::command_to_server::StringToServer;
use crate::utils::{generate_random_string, handle_passwd, Error, Result};
use database::Database;
use fs::DATA_DB;
use services::args::parse as parse_args;
use std::collections::HashMap;
use std::io::{Error as IOError, ErrorKind};
use std::path;
use chrono::Utc;
use crate::fs::SOCKET_ADDR;
use crate::models::response::Response;
use crate::services::socket::Socket;

pub mod cli {
    pub const DIVISOR : &str = "|";
}

pub mod fs {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    pub const DATA_FOLDER : &str = ".pocket";
    pub const DATA_DB : &str = "pocket-cli.db";
    pub const SOCKET_PORT : u16 = 8300;
    pub const SOCKET_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), SOCKET_PORT);
}

pub struct Pocket {
    database: Database,
    pub property: Option<Property>,
    pub socket: Option<Socket>,
    error: Option<Error>
}


impl Pocket {

    pub fn new(mut file_db_path: String) -> Self {

        file_db_path.push(path::MAIN_SEPARATOR);
        file_db_path.push_str(DATA_DB);

        let mut ret = Pocket {
            database: Database::new(),
            property: None,
            socket: None,
            error: None
        };

        if let Err(e) = ret.database.init(file_db_path) {
            ret.error = Some(Error::Msg(e.to_string()));
        }

        match ret.database.execute::<Property>("SELECT * FROM properties WHERE _key = \"login\"") {
            Ok(properties) => {
                if !properties.is_empty() {
                    if let Some(p) = properties.first() {
                        ret.property = Some(p.to_owned());
                    }
                }
            }
            Err(e) => ret.error = Some(Error::Msg(e.to_string()))
        }
        
        ret
    }

    fn insert_iv_property(&mut self) -> Option<Vec<u8>> {
        let iv = generate_random_string(16);

        let mut property = Property::new(1, 0, "iv".to_string(), iv.clone(), Utc::now().timestamp());

        if self.database.update::<Property>("INSERT INTO properties (server_id, _key, _value, timestamp) VALUES (?1, ?2, ?3, ?4)", &mut property) {
            Some(iv.as_bytes().to_vec())
        } else {
            None
        }
    }
    
    pub fn login_server(&mut self, passwd_opt: Option<String>) -> Result<(), IOError> {

        let iv = match self.database.execute::<Property>("SELECT * FROM properties WHERE _key = \"iv\"") {
            Ok(properties) => {
                if !properties.is_empty() {
                    if let Some(p) = properties.first() {
                        Some(p.value.as_bytes().to_vec())
                    } else {
                        self.insert_iv_property()
                    }
                } else {
                    self.insert_iv_property()
                }
            }
            Err(_) => self.insert_iv_property()
        };
        
        let passwd = match passwd_opt {
            None => match &self.property {
                None => return Err(IOError::new(ErrorKind::Other, "No password provided.")),
                Some(property) => match handle_passwd(&property.value, &iv, false) {
                    Err(e) => {
                        self.database.delete("DELETE FROM properties WHERE _key = \"login\"");
                        return Err(IOError::new(ErrorKind::Other, e))
                    }
                    Ok(pwd) => {
                        let mut ret = " ".to_string();
                        ret.push_str(pwd.as_str());
                        ret
                    }
                }
            }
            Some(pwd) => {
                let mut ret = " ".to_string();
                ret.push_str(pwd.as_str());
                
                
                self.database.delete("DELETE FROM properties WHERE _key = \"login\"");
                let mut property = Property::new(1, 0, "login".to_string(), pwd, Utc::now().timestamp());

                if !self.database.update::<Property>("INSERT INTO properties (server_id, _key, _value, timestamp) VALUES (?1, ?2, ?3, ?4)", &mut property) {
                    return Err(IOError::new(ErrorKind::Other,"Impossible insert property"))
                }
                
                ret
            }
        };
        
        let mut socket = match  Socket::connect(SOCKET_ADDR.to_string()) {
            Ok(socket) => socket, 
            Err(e) => return Err(e)
        };
        
        match socket.write(&passwd) {
            Ok(ret) => 
                match Response::to_response(&ret.trim().to_string()) {
                    Response::Ok => {

                        self.database.delete("DELETE FROM properties WHERE _key = \"login\"");

                        let pwd = match handle_passwd(&passwd, &iv, true) {
                            Ok(pwd) => pwd.to_string(),
                            Err(err) => return Err(IOError::new(ErrorKind::Other, err))
                        };
                        
                        let mut property = Property::new(1, 0, "login".to_string(), pwd, Utc::now().timestamp());
                        
                        if !self.database.update::<Property>("INSERT INTO properties (server_id, _key, _value, timestamp) VALUES (?1, ?2, ?3, ?4)", &mut property) {
                            return Err(IOError::new(ErrorKind::Other,"Impossible insert property"))
                        }
                        
                        self.socket = Some(socket);
                        Ok(())
                    }
                    other => Err(IOError::new(ErrorKind::Other, other.to_string())) 
                }
            Err(e) => Err(IOError::new(ErrorKind::Other, e.to_string()))
        }
    }
    
    pub fn execute(&mut self, model: impl StringToServer) -> Result<String, IOError> {

        let socket = match self.socket.as_mut() {
            Some(socket) => socket,
            None => return Err(IOError::new(ErrorKind::Other, "Connection issue".to_string()))
        };

        let str = socket.write(&model.get_string_to_sever())?;
        Ok(match Response::to_response(&str) {
            Response::Ok => "OK".to_string(),
            Response::Error => str,
            resp => resp.to_string()
        })
    }

    pub fn parse<F>(&self, args: &Vec<String>, parse: F) -> (Option<CliCommands>, Result<HashMap<&'static str, CliOptions>, Error>)
    where F: Fn(&Vec<String>) -> Result<HashMap<&'static str, CliOptions>, Error> {
        
        if let Some(commands) = parse_args(args) {
            (Some(commands), parse(args))            
        } else {
            (None, Err(Error::Undefine))
        }
        
    }
}




