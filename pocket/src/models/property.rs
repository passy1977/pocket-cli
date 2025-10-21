
#[derive(Clone)]
pub struct Property {
    pub id: i64,
    pub server_id: i64,
    pub key: String,
    pub value: String,
    pub timestamp: i64
}

impl Property {
    pub fn new(id: i64, server_id: i64, key: String, value: &String, timestamp: i64) -> Self {
        Property {
            id,
            server_id,
            key,
            value: value.clone(),
            timestamp
        }
    }
}
