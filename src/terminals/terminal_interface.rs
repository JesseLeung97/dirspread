use crate::Dirspread;
use std::error::Error;

pub trait TerminalInterface {
    fn open_terminals(dirspread: &Dirspread) -> Result<(), Box<dyn Error>>;
}
