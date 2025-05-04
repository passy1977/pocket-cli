use rusqlite::Statement;

pub trait DatabaseWrite {

    fn write(&self, statement: &mut Statement) -> rusqlite::Result<()>;

}
