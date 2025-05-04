pub mod models;
pub mod services;
pub mod utils;
pub mod traits;
mod database;

use crate::models::commands::{CliCommands, CliOptions};
use crate::models::property::Property;
use crate::traits::command_to_server::StringToServer;
use crate::utils::{Error, Result};
use chrono::Utc;
use database::Database;
use fs::DATA_DB;
use mac_address::get_mac_address;
use services::args::parse as parse_args;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path;
use crate::fs::SOCKET_ADDR;

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
    stream: Option<TcpStream>,
    pub logged: bool,
    error: Option<Error>
}


impl Pocket {

    pub fn new(mut file_db_path: String) -> Self {

        file_db_path.push(path::MAIN_SEPARATOR);
        file_db_path.push_str(DATA_DB);

        let mut ret = Pocket {
            database: Database::new(),
            stream: None,
            logged: false,
            error: None
        };

        if let Err(e) = ret.database.init(file_db_path) {
            ret.error = Some(Error::Msg(e.to_string()));
        }

        match ret.database.execute::<Property>("SELECT * FROM properties WHERE _key = \"login\"") {
            Ok(properties) => {
                if !properties.is_empty() {
                    ret.logged = true
                }
            }
            Err(e) => ret.error = Some(Error::Msg(e.to_string()))
        }

        ret
    }


    fn encrypt_passwd(&mut self, passwd: &String) -> Option<String> {
        
        let mut aad :  [u8; 32]= ['$' as u8; 32]; 
        
        let mac_in_bytes  = match get_mac_address() {
            Ok(Some(ma)) => ma.bytes(),
            Ok(None) => {
                self.error = Some(Error::Msg("No MAC address found.".to_string()));
                return None
            }
            Err(_) => return None
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
    
    pub fn login_server(&mut self, passwd: String) -> Result<()> {
        if let Ok(mut stream) = TcpStream::connect(SOCKET_ADDR.to_string()) {
            
            //socket.set_read_timeout(Duration::new(1, 0)).unwrap();
            stream.write(&[0]).expect("TODO: panic message");

            let mut buffer = String::new();

            // let pwd = passwd.as_bytes();
            //
            // stream.write_all(pwd).expect("TODO: panic message 1");
            //
            match stream.read_to_string(&mut buffer) {
                Ok(bytes_read) => println!("{bytes_read} {}", buffer),
                Err(e) => println!("{}", e)
            }

            self.stream = Some(stream);
        } else {
            return Err("Connection to Pocket server failed.")
        }

        let mut prop = Property::new(1, 0, "login".to_string(), self.encrypt_passwd(&passwd).unwrap(), Utc::now().timestamp());

        self.database.delete("DELETE FROM properties WHERE _key = \"login\"");
        
        if !self.database.update::<Property>("INSERT INTO properties (server_id, _key, _value, timestamp) VALUES (?1, ?2, ?3, ?4)", &mut prop) {
            return Err("Impossible insert property");    
        }
        
        Ok(())
    }
    
    pub fn execute(&mut self, model: impl StringToServer) -> Result<String> {

        let mut buffer = String::new();
        
        match self.stream {
            None => {
                if let Ok(stream) = TcpStream::connect(SOCKET_ADDR.to_string()) {
                    self.stream = Some(stream);
                } else {
                    return Err("Connection to Pocket server failed.")
                }
            }
            _ => {}
        }
        
        if let Some(ref mut stream) = self.stream {
            
            match stream.write_all(model.get_string_to_sever().as_bytes()) {
                Ok(_) => {}
                Err(e) => self.error = Some(Error::Msg(e.to_string()))
            }
            
            match stream.read_to_string(&mut buffer) {
                Ok(bytes_read) => println!("{bytes_read} {}", buffer),
                Err(e) => self.error = Some(Error::Msg(e.to_string()))
            }
            
        } else {
            return Err("Connection to Pocket server failed.")
        }


        Ok(buffer)
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




