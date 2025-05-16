use rusqlite::{Row, Statement};
use crate::database::database_read::DatabaseRead;
use crate::database::database_write::DatabaseWrite;
use crate::models::property::Property;

impl DatabaseRead<Property> for Property {
    fn read(row: &Row) -> Property {
        Property {
            id: row.get(0).unwrap(),
            server_id: row.get(1).unwrap(),
            key: row.get(2).unwrap(),
            value: row.get(3).unwrap(),
            timestamp: row.get(4).unwrap(),
        }
    }
}

impl DatabaseWrite for Property {
    fn write(&self, statement: &mut Statement) -> rusqlite::Result<()>  {
        statement.raw_bind_parameter(1, &self.server_id)?;
        statement.raw_bind_parameter(2, &self.key)?;
        statement.raw_bind_parameter(3, &self.value)?;
        statement.raw_bind_parameter(4, &self.timestamp)?;
        Ok(())
    }
}