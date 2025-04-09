use rusqlite::Statement;

pub trait DatabaseRead<T> {
    
    fn read(&mut self, statement: &Statement) -> T;

}
