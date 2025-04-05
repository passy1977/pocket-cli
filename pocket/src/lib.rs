use crate::models::model::Model;

pub mod constants;
pub mod models;
pub mod services;
pub mod utils;
mod database;

use database::Database;

pub struct Pocket {
    database: Database,
}


impl Pocket {

    pub fn new(file_db_path: String) -> Self {
        let mut ret = Pocket {
            database: Database::new()
        };

        ret.database.init(file_db_path);

        return ret;
    }

    pub fn login_server(&self, passwd: String) -> Result<String, String>  {

        Ok("".to_string())
    }

    pub fn execute(&self, model: impl Model) -> Result<String, String> {


        Ok("".to_string())
    }
}




