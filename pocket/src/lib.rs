pub mod models;
pub mod services;
pub mod utils;
pub mod traits;
mod database;

use std::collections::HashMap;
use std::io::{Read, Write};
use fs::{DATA_DB, SOCKET_PORT};
use crate::traits::command_to_server::StringToServer;
use crate::utils::{Result, Error};
use crate::models::commands::{CliCommands, CliOptions};
use database::Database;
use services::args::parse as parse_args;
use std::path;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use chrono::{Duration, Utc};
use crate::models::property::Property;
use mac_address::get_mac_address;

pub mod cli {
    pub const DIVISOR : &str = "|";
}
pub mod fs {
    pub const DATA_FOLDER : &str = ".pocket";
    pub const DATA_DB : &str = "pocket-cli.db";
    pub const SOCKET_PORT : u16 = 8300;
}

pub struct Pocket {
    database: Database,
    pub logged: bool,
    error: Option<Error>
}


impl Pocket {

    pub fn new(mut file_db_path: String) -> Self {

        file_db_path.push(path::MAIN_SEPARATOR);
        file_db_path.push_str(DATA_DB);

        let mut ret = Pocket {
            database: Database::new(),
            logged: false,
            error: None
        };

        if let Err(e) = ret.database.init(file_db_path) {
            ret.error = Some(Error::Msg(e.to_string()));
        }
        
        ret
    }


    fn encrypt_passwd(passwd: &String) -> Option<String> {
        
        let mut aad :  [u8; 32]= ['$' as u8; 32]; 
        
        let mac_in_bytes  = match get_mac_address() {
            Ok(Some(ma)) => ma.bytes(),
            Ok(None) => { 
                println!("No MAC address found.");
                return None
            }
            Err(e) => return None
        };

        aad[..mac_in_bytes.len()].copy_from_slice(&mac_in_bytes);

        tink_aead::init();
        let kh = tink_core::keyset::Handle::new(&tink_aead::aes256_gcm_key_template()).ok().unwrap();
        if let Ok(a) = tink_aead::new(&kh) {
            let ct = a.encrypt(passwd.as_bytes(), &aad).ok().unwrap();
            Some(hex::encode(&ct))
        } else {
            None
        }
    }
    
    pub fn login_server(&mut self, passwd: String) -> Result<&'static str>  {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), SOCKET_PORT);

        let socket = TcpStream::connect(socket.to_string());
        
        if let Ok(mut socket) = socket {
            //socket.set_read_timeout(Duration::new(1, 0)).unwrap();
            socket.write(&[0]);

            socket.write(passwd.as_bytes());

        } else {
            return Err("Connection to Pocket server failed.")
        }

        
        
        let mut prop = Property::new(1, 0, "login".to_string(), Pocket::encrypt_passwd(&passwd).unwrap(), Utc::now().timestamp());

        if !self.database.delete("DELETE FROM properties WHERE key = \"login\"") {
            return Err("Impossible delete property");
        }
        
        if !self.database.update::<Property>("INSERT INTO properties (id, server_id, key, value, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)", &mut prop) {
            return Err("Impossible insert property");    
        }
        
        Ok("")
    }
    
    pub fn execute(&self, model: impl StringToServer) -> Result<String, String> {


        Ok(model.get_string_to_sever())
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




