use rusqlite::Statement;

pub trait DatabaseWrite {
    
    #[allow(dead_code)]
    fn write(&self, statement: &mut Statement) -> rusqlite::Result<()>;

}
