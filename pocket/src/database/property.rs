use sqlite3::Statement;
use crate::database::database_read::DatabaseRead;
use crate::models::property::Property;

impl DatabaseRead<Property> for Property {
    fn read(&mut self, statement: &Statement) -> Property {
        Property {
            id: statement.read::<i64>(0).unwrap(),
            server_id: statement.read::<i64>(1).unwrap(),
            key: statement.read::<String>(2).unwrap(),
            value: statement.read::<String>(3).unwrap(),
            timestamp: statement.read::<i64>(4).unwrap()
        }
    }
}
