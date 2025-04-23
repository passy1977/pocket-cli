use rusqlite::{Row, Error};

pub trait DatabaseRead<T> {
    
    fn read(statement: &Row) -> T;

}
