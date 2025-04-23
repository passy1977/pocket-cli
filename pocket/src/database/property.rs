use rusqlite::{Row, Statement};
use crate::database::database_read::DatabaseRead;
use crate::database::database_write::DatabaseWrite;
use crate::models::property::Property;
//
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
//

impl DatabaseWrite for Property {
    fn write(&self, statement: &mut Statement) {
        // statement.bind(1, self.id).unwrap();
        // statement.bind(2, self.server_id).unwrap();
        // statement.bind(3, self.key.as_str()).unwrap();
        // statement.bind(4, self.value.as_str()).unwrap();
        // statement.bind(4, self.timestamp).unwrap();
    }
}