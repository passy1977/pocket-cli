use std::path;
use crate::constants::fs::DATA_DB;
use crate::models::model::Model;

pub mod constants;
pub mod models;
pub mod services;
pub mod utils;
mod database;

use database::Database;
use crate::utils::Error;

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

        if let Err(err) = ret.database.init(file_db_path) {
            ret.error = Some(err);
        }

       // ret.database.execute::<Property>("SELECT * FROM properties", ());

        ret
    }

    pub fn login_server(&self, passwd: String) -> Result<String, String>  {

        Ok("".to_string())
    }

    pub fn execute(&self, model: impl Model) -> Result<String, String> {


        Ok("".to_string())
    }
}




