use crate::models::commands::Commands;

pub trait Model {
    fn cmd(&self) -> Commands;
}