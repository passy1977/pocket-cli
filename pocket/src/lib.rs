pub mod constants;
pub mod models;
pub mod services;
pub mod utils;
mod database;

use std::path;
use crate::constants::fs::{DATA_DB, SOCKET_PORT};
use crate::models::model::Model;
use crate::utils::Error;
use database::Database;
use services::args::parse;
use crate::models::commands::{CliCommands, CliOptions};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

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

    pub fn login_server(&mut self, passwd: String) -> Result<&'static str, &'static str>  {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), SOCKET_PORT);

        if let None = self.socket {
            return Err("Connection to Pocket server failed.")
        }

        self.socket = Some(socket);
        Ok("")
    }
    
    pub fn execute(&self, model: impl Model) -> Result<String, String> {


        Ok("".to_string())
    }

    pub fn parse(&self) -> (Option<CliCommands>, Vec<CliOptions>) {
        parse()
    }
}




