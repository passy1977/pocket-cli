pub mod database_read;
pub mod database_write;
pub mod property;

use rusqlite::{Connection, Params};
use crate::utils::{Error::Message, Result};
use crate::database::database_read::DatabaseRead;
use crate::database::database_write::DatabaseWrite;
use crate::models::property::Property;

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

        }
    } 

    pub fn init(&mut self, file_db_path: String) -> Result<()> {


        self.connection = match  Connection::open(file_db_path) {
            Ok(connection) => Some(connection),
            Err(err) => {
                return Err(Message(err.to_string()))
            }
        };

        if !self.is_created() {
            if let Err(e) = self.create() {
                return Err(Message(e.to_string()))
            }
        }
        
        Ok(())
    } 

    fn create(&self) -> Result<bool> {
        if let Some(ref connection) = self.connection {
            return match connection.prepare(CREATION_SQL) {
                Ok(_) => Ok(true),
                Err(err) => Err(Message(err.to_string()))
            }
        }
        Ok(false)
    }

    pub fn is_created(&self) -> bool {
        if let Some(ref connection) = self.connection {
            connection.execute("SELECT * FROM properties", ()).is_ok()
        } else {
            false
        }
    }


    pub fn execute<T, P: Params>(&self, sql: &str, params: P) -> Result<Vec<T>> {

        //database_read: &mut impl DatabaseRead<T>,

        if let Some(ref conn) = self.connection {
            let mut ret : Vec<T> = Vec::new();
            if let Ok(mut stmt) = conn.prepare("SELECT id, name, data FROM person") {
                let _ = stmt.query_map([], |row| {
                    Ok(<Property as DatabaseRead<Property>>::read(row))
                });
            }



        //
        //     let mut statement = connection
        //         .prepare(sql)
        //         .unwrap();
        //
        //     // while let State::Row = statement.next().unwrap() {
        //     //     ret.push(database_read.read(&statement));
        //     // }
        //
             Ok(ret)
        } else {
            Err(Message(String::from("Database connection does not exist")))
        }
    }

    pub fn update<T>(&self, database_write: &mut impl DatabaseWriteù, sql: &str) -> usize {
        // if let Some(ref connection) = self.connection {
        //     let mut ret : Vec<T> = Vec::new();
        //
        //     let mut statement = connection
        //         .prepare(sql)
        //         .unwrap();
        //
        //     database_write.write(&mut statement);
        //
        //     0
        // } else {
        //     0
        // }
        0
    }
}