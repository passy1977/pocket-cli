pub mod models;
pub mod services;
pub mod utils;
pub mod traits;
mod database;

use crate::models::commands::{CliCommands, CliOptions};
use crate::models::property::Property;
use crate::traits::command_to_server::StringToServer;
use crate::utils::{Error, Result};
use database::Database;
use fs::DATA_DB;
use mac_address::get_mac_address;
use services::args::parse as parse_args;
use std::collections::HashMap;
use std::io::{Error as IOError, ErrorKind};
use std::path;
use crate::fs::{Response, SOCKET_ADDR};
use crate::services::socket::Socket;

pub mod cli {
    pub const DIVISOR : &str = "|";
}
pub mod fs {
    use std::cmp::PartialEq;
    use std::fmt::Display;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    pub const DATA_FOLDER : &str = ".pocket";
    pub const DATA_DB : &str = "pocket-cli.db";
    pub const SOCKET_PORT : u16 = 8300;
    pub const SOCKET_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), SOCKET_PORT);

    #[derive(Clone)]
    pub(super) enum Response {
        Ok = 0,
        Error = 1,
        WrongParams = 2,
        UserAlreadyExist = 3,
        DeviceAlreadyExist = 4,
        UserNotExist = 5,
        DeviceNotExist = 6,
        WrongPasswd = 7
    }
    impl Display for Response {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Response::*;
            
            match *self {
                Ok => write!(f, "Ok({})", self.clone() as u8),
                Error => write!(f, "Error({})", self.clone() as u8),
                WrongParams => write!(f, "WrongParams({})", self.clone() as u8),
                UserAlreadyExist => write!(f, "UserAlreadyExist({})", self.clone() as u8),
                DeviceAlreadyExist => write!(f, "DeviceAlreadyExist({})", self.clone() as u8),
                UserNotExist => write!(f, "UserNotExist({})", self.clone() as u8),
                DeviceNotExist => write!(f, "DeviceNotExist({})", self.clone() as u8),
                WrongPasswd => write!(f, "WrongPasswd({})", self.clone() as u8),
            }
        }
    }

    impl PartialEq for Response {
        fn eq(&self, other: &Self) -> bool {
            self.clone() as u8 == other.clone() as u8
        }
    }

    impl Response {

        pub(super) fn check(&self, str: &String ) -> bool {
            use Response::*;
            
            match self {
                x if *x == Ok && str == "0" => true,
                x if *x == Error && str == "1" => true,
                x if *x == WrongParams && str == "2" => true,
                x if *x == UserAlreadyExist && str == "3" => true,
                x if *x == DeviceAlreadyExist && str == "4" => true,
                x if *x == UserNotExist && str == "5" => true,
                x if *x == DeviceNotExist && str == "6" => true,
                x if *x == WrongPasswd && str == "7" => true,
                _ => false
            }
        }

        pub(super) fn to_response(str: &String ) -> Self {
            use Response::*;

            match str.as_str() {
                _x @ "0" => Ok,
                _x @ "1" => Error,
                _x @ "2" => WrongParams,
                _x @ "3" => UserAlreadyExist,
                _x @ "4" => DeviceAlreadyExist,
                _x @ "5" => UserNotExist,
                _x @ "6" => DeviceNotExist,
                _x @ "7" => WrongPasswd,
                _ => Error
            }
        }
    }
    
    
}

pub struct Pocket {
    database: Database,
    pub property: Option<Property>,
    error: Option<Error>
}


impl Pocket {

    pub fn new(mut file_db_path: String) -> Self {

        file_db_path.push(path::MAIN_SEPARATOR);
        file_db_path.push_str(DATA_DB);

        let mut ret = Pocket {
            database: Database::new(),
            property: None,
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


    fn handle_passwd(passwd: &String, encode: bool) -> Result<String> {
        
        let mut aad :  [u8; 32]= ['$' as u8; 32]; 
        
        let mac_in_bytes  = match get_mac_address() {
            Ok(Some(ma)) => ma.bytes(),
            Ok(None) => return Err("Mac not found"),
            Err(_) => return Err("Error in translation"),
        };

        aad[..mac_in_bytes.len()].copy_from_slice(&mac_in_bytes);

        tink_aead::init();
        let kh = tink_core::keyset::Handle::new(&tink_aead::aes256_gcm_key_template()).ok().unwrap();
        if let Ok(a) = tink_aead::new(&kh) {
            Ok(
                if encode {
                    if let Ok(ec) = a.encrypt(passwd.as_bytes(), &aad) {
                        hex::encode(&ec)
                    } else {
                        return Err("Encode error")
                    }
                } else {
                    if let Ok(dc) = a.decrypt(passwd.as_bytes(), &aad) {
                        String::from_utf8(dc).expect("Unable convert password to utf8")    
                    } else {
                        return Err("Decode error")
                    }
                }
            )
        } else {
            Err("Keyset error")
        }
    }

    pub fn login_server(&mut self, passwd_opt: Option<String>) -> Result<(), IOError> {
        
        let passwd = match passwd_opt {
            None => match &self.property {
                None => return Err(IOError::new(ErrorKind::Other, "No password provided.")),
                Some(prop) => match Pocket::handle_passwd(&prop.value, false) {
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
                ret
            }
        };
        
        let mut socket = if let Ok(socket) = Socket::connect(SOCKET_ADDR.to_string()) {
            socket
        } else {
            return Err(IOError::new(ErrorKind::Other, "No password provided."))
        };

        
        match socket.write(&passwd) {
            Ok(ret) => 
                match Response::to_response(&ret.trim().to_string()) {
                    Response::Ok => Ok(()),
                    x => Err(IOError::new(ErrorKind::Other, x.to_string())) 
                }
            Err(e) => Err(IOError::new(ErrorKind::Other, e.to_string()))
        }
    }
    
    pub fn execute(&mut self, model: impl StringToServer) -> Result<String> {

        let mut buffer = String::new();
        
        // match self.stream {
        //     None => {
        //         if let Ok(mut stream) = TcpStream::connect(SOCKET_ADDR.to_string()) {
        //
        //             let mut reader = BufReader::new(&mut stream);
        //
        //             let received: Vec<u8> = reader.fill_buf().expect("").to_vec();
        //
        //             reader.consume(received.len());
        //
        //             self.stream = Some(stream);
        //         } else {
        //             return Err("Connection to Pocket server failed.")
        //         }
        //     }
        //     _ => {}
        // }
        //
        // if let Some(ref mut stream) = self.stream {
        //
        //     match stream.write_all(model.get_string_to_sever().as_bytes()) {
        //         Ok(_) => stream.flush().expect(""),
        //         Err(e) => self.error = Some(Error::Msg(e.to_string()))
        //     }
        //
        //     match stream.read_to_string(&mut buffer) {
        //         Ok(bytes_read) => println!("{bytes_read} {}", buffer),
        //         Err(e) => self.error = Some(Error::Msg(e.to_string()))
        //     }
        //
        // } else {
        //     return Err("Connection to Pocket server failed.")
        // }


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




