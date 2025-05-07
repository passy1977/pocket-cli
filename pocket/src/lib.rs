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
use std::io::{BufRead, BufReader, Read, Write, Error as IOError, ErrorKind, BufWriter};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::{io, path, thread};
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
                    Ok(pwd) => pwd
                }
            }
            Some(pwd) => pwd
        };

        let mut stream_reader = match TcpStream::connect(SOCKET_ADDR.to_string()) {
            Ok(stream) => stream,
            Err(_) => return Err(IOError::new(ErrorKind::Other,"Connection to Pocket server failed"))
        };

        let mut stream_writer = match stream_reader.try_clone() {
            Ok(stream) => stream,
            Err(_) => return Err(IOError::new(ErrorKind::Other,"Cloning stream error"))
        };

        thread::spawn(move || {
            loop {
                let mut buffer = [0; 1024];
                match stream_reader.read(&mut buffer) {
                    Ok(bytes_read) if bytes_read > 0 => {
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println!("Risposta dal server: {}", response);
                    }
                    Ok(_) => {
                        println!("Nessun dato ricevuto.");
                    }
                    Err(e) => {
                        println!("Errore durante la lettura: {}", e);
                    }
                }
            }
        });


        loop {
            // Leggi un messaggio dall'input dell'utente
            let mut input = String::new();
            println!("Inserisci un messaggio (o 'exit' per uscire):");
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            // Esci dal ciclo se l'utente scrive 'exit'
            if input.eq_ignore_ascii_case("exit") {
                break;
            }

            // Invia il messaggio al server
            stream_writer.write_all(input.as_bytes())?;

            // Ricevi la risposta dal server
            // let mut buffer = [0; 1024];
            // match stream.read(&mut buffer) {
            //     Ok(bytes_read) if bytes_read > 0 => {
            //         let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            //         println!("Risposta dal server: {}", response);
            //     }
            //     Ok(_) => {
            //         println!("Nessun dato ricevuto.");
            //     }
            //     Err(e) => {
            //         println!("Errore durante la lettura: {}", e);
            //     }
            // }
        }




        // Chiudi la connessione

        

        // let stream_reader = match TcpStream::connect(SOCKET_ADDR.to_string()) {
        //     Ok(stream) => stream,
        //     Err(_) => return Err(IOError::new(ErrorKind::Other,"Connection to Pocket server failed"))
        // };
        // 
        // let mut stream_writer = match stream_reader.try_clone() {
        //     Ok(stream) => stream,
        //     Err(_) => return Err(IOError::new(ErrorKind::Other,"Cloning stream error"))
        // };
        // 
        // let mut reader = BufReader::new(&stream_reader);
        // let mut writer = BufWriter::new(&stream_writer);
        // 
        // writer.write(&[0])?;
        // 
        // writer.write_all(b"\n")?;
        // 
        // writer.write(&passwd.as_bytes())?;
        // 
        // writer.write_all(b"\n")?;
        // 
        // stream_writer.shutdown(std::net::Shutdown::Write)?;
        // 
        // let mut buffer = String::new();
        // 
        // reader.read_to_string(&mut buffer)?;
        // 
        // println!("{}", buffer);


        //
        // stream_writer.shutdown(Shutdown::Both)?;
        //
        // let mut received: Vec<u8> = vec![];
        //
        // // Array with a fixed size
        // let mut rx_bytes = [0u8; 5 * 1_024];
        // loop {
        //     // Read from the current data in the TcpStream
        //     let bytes_read = stream_reader.read(&mut rx_bytes)?;
        //
        //     // However many bytes we read, extend the `received` string bytes
        //     received.extend_from_slice(&rx_bytes[..bytes_read]);
        //
        //     // If we didn't fill the array
        //     // stop reading because there's no more data (we hope!)
        //     if bytes_read < rx_bytes.len() {
        //         break;
        //     }
        // }
        //
        // let a = String::from_utf8(received);
        //
        // let mut prop = Property::new(1, 0, "login".to_string(), Pocket::handle_passwd(&passwd, true).unwrap(), Utc::now().timestamp());
        //
        // self.database.delete("DELETE FROM properties WHERE _key = \"login\"");
        //
        // if !self.database.update::<Property>("INSERT INTO properties (server_id, _key, _value, timestamp) VALUES (?1, ?2, ?3, ?4)", &mut prop) {
        //     return Err(IOError::new(ErrorKind::Other,"Impossible insert property"))
        // }
        //
        // self.property = Some(prop);
        //
        Ok(())
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




