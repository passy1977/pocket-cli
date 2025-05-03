pub mod database_read;
pub mod database_write;
pub mod property;

use rusqlite::Connection;
use crate::utils::Result;
use crate::database::database_read::DatabaseRead;
use crate::database::database_write::DatabaseWrite;

const CREATION_SQL : &str = r#"
CREATE TABLE `properties` ( `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, server_id integer NOT NULL DEFAULT 0, `_key` TEXT NOT NULL DEFAULT '', `_value` TEXT NOT NULL DEFAULT '', timestamp integer NOT NULL DEFAULT 0);
"#;

pub struct Database {
    connection: Option<Connection>
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

    pub fn init(&mut self, file_db_path: String) -> Result<(), String> {

        self.connection = match  Connection::open(file_db_path) {
            Ok(connection) => Some(connection),
            Err(e) => return Err(e.to_string())
        };

        if !self.is_created() {
            if let Err(e) = self.create() {
                return Err(e.to_string());
            }
        }
        
        Ok(())
    } 

    fn create(&self) -> Result<bool, String> {
        if let Some(ref connection) = self.connection {
            return match connection.execute(CREATION_SQL, []) {
                Ok(_) => Ok(true),
                Err(e) => Err(e.to_string())
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


    pub fn execute<T>(&self, sql: &str) -> Result<Vec<T>>
    where
        T: DatabaseRead<T>
    {

        if let Some(ref conn) = self.connection {
            let mut ret : Vec<T> = Vec::new();

            if let Ok(mut query) = conn.prepare(sql)
            {
                if let Ok(mut rows) = query.query(()) {
                    while let Ok(Some(row)) = rows.next() {
                        ret.push(<T as DatabaseRead<T>>::read(&row));
                    }
                }
            } else {
                return Err("Sql error")    
            }
            
            Ok(ret)
        } else {
            Err("Database connection does not exist")
        }
    }

    pub fn update<T>(&self, sql: &str, database_write: &mut impl DatabaseWrite) -> bool {
        if let Some(ref connection) = self.connection {
            
            let mut statement = connection
                .prepare(sql)
                .unwrap();
        
            database_write.write(&mut statement);

            true
        } else {
            false
        }
    }

    pub fn delete(&self, sql: &str) -> bool {
        if let Some(ref connection) = self.connection {

            let _ = connection
                .prepare(sql)
                .unwrap();
            
            true
        } else {
            false
        }
    }
}