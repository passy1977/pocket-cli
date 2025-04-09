use rusqlite::Statement;

pub trait DatabaseWrite<T> {

    fn write(&self, statement: &mut Statement);

}
