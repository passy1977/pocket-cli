use rusqlite::Row;

pub trait DatabaseRead<T> {
    
    fn read(statement: &Row) -> T;

}
