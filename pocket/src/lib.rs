pub mod models;
pub mod services;
pub mod utils;
pub mod traits;
mod database;

use std::collections::HashMap;
use fs::{DATA_DB, SOCKET_PORT};
use crate::traits::command_to_server::StringToServer;
use crate::utils::{Result, Error};
use crate::models::commands::{CliCommands, CliOptions};
use database::Database;
use services::args::parse as parse_args;
use std::path;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use chrono::Utc;
use crate::models::property::Property;

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
    socket: Option<SocketAddr>,
    pub logged: bool,
    error: Option<Error>
}


impl Pocket {

    pub fn new(mut file_db_path: String) -> Self {

        file_db_path.push(path::MAIN_SEPARATOR);
        file_db_path.push_str(DATA_DB);

        let mut ret = Pocket {
            database: Database::new(),
            socket: None,
            logged: false,
            error: None
        };

        if let Err(e) = ret.database.init(file_db_path) {
            ret.error = Some(Error::Msg(e.to_string()));
        }

        ret
    }


    fn get_cpu_serial_number() -> Option<String> {
        let cpu_id = CpuId::new();
        if cpu_id.has_feature_info() && cpu_id.feature_info().has_serial_number() {
            let serial_number = cpu_id.feature_info().serial_number();
            Some(serial_number.to_string())
        } else {
            None
        }
    }
    
    pub fn login_server(&mut self, passwd: String) -> Result<&'static str>  {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), SOCKET_PORT);

        if let None = self.socket {
            return Err("Connection to Pocket server failed.")
        }

        self.socket = Some(socket);
        
        let mut prop = Property::new(1, 0, "login".to_string(), "".to_string(), Utc::now().timestamp());

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
            (parse_args(args), parse(args))            
        } else {
            (None, Err(Error::Undefine))
        }
        
    }
}




