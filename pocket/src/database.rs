pub mod database_read;
pub mod property;

use crate::utils::{Error::Message, Result};
use sqlite3::{Connection, State};
use crate::database::database_read::DatabaseRead;

const CREATION_SQL : &str = r#"
CREATE TABLE `properties` ( `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, server_id integer NOT NULL DEFAULT 0, `_key` TEXT NOT NULL DEFAULT '', `_value` TEXT NOT NULL DEFAULT '', timestamp integer NOT NULL DEFAULT 0);
"#;

#[derive(Clone)]
pub enum Status {
    Ok,
    Error,
    Empty
}

impl PartialEq<Status> for Status {
    fn eq(&self, other: &Status) -> bool {
        self.clone() as u8 == other.clone() as u8
    }
}

pub struct Database {
    connection: Option<Connection>,
    init: bool
}

impl std::fmt::Debug for Database {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}


impl Database {

    pub fn new() -> Self {
        Database {
            connection: None,
            init:false
        }
    } 

    pub fn init(&mut self, file_db_path: String) -> Result<()> {


        self.connection = match sqlite3::open(&file_db_path) {
            Ok(connection) => Some(connection),
            Err(err) => {
                return Err(Message(err.message.unwrap()))
            }
        };

        if !self.is_created() {
            self.init = self.create(&file_db_path);
        }
        
        Ok(())
    } 

    fn create(&self, _file_db_path: &String) -> bool {
        if let Some(ref connection) = self.connection {
            if let Ok(()) = connection.execute(CREATION_SQL) {
                return true
            }
        }
        false
    }

    fn is_created(&self) -> bool {
        if let Some(ref connection) = self.connection {
            connection.iterate("SELECT * FROM properties", | _ | true ).is_ok()
        } else {
            false
        }
    }


    fn execute<T>(&self, database_read: &mut impl DatabaseRead<T>, sql: &str) -> Result<Vec<T>> {
        if let Some(ref connection) = self.connection {
            let mut ret : Vec<T> = Vec::new();

            let mut statement = connection
                .prepare(sql)
                .unwrap();

            while let State::Row = statement.next().unwrap() {
                ret.push(database_read.read(&statement));
            }

            Ok(ret)
        } else {
            Err(Message(String::from("Database connection does not exist")))
        }
    }
}