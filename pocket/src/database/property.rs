use rusqlite::Statement;
use crate::database::database_read::DatabaseRead;
use crate::database::database_write::DatabaseWrite;
use crate::models::property::Property;
//
impl DatabaseRead<Property> for Property {
    fn read(&mut self, statement: &Statement) -> Property {
//         Property {
//             id: statement.read::<i64>(0).unwrap(),
//             server_id: statement.read::<i64>(1).unwrap(),
//             key: statement.read::<String>(2).unwrap(),
//             value: statement.read::<String>(3).unwrap(),
//             timestamp: statement.read::<i64>(4).unwrap()
//         }
        
        Property {
            id: 0,
            server_id: 0,
            key: "".to_string(),
            value: "".to_string(),
            timestamp: 0,
        }
    }
}
//
impl DatabaseWrite<Property> for Property {
    fn write(&self, statement: &mut Statement) {
//         statement.bind(1, self.id).unwrap();
//         statement.bind(2, self.server_id).unwrap();
//         statement.bind(3, self.key.as_str()).unwrap();
//         statement.bind(4, self.value.as_str()).unwrap();
//         statement.bind(4, self.timestamp).unwrap();
    }
}