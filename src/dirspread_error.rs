use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DirspreadError {
    pub msg: String
}

impl DirspreadError {
    pub fn new(msg: &str) -> DirspreadError {
        DirspreadError { 
            msg: msg.to_string() 
        }
    }
}

impl fmt::Display for DirspreadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for DirspreadError {
    fn description(&self) -> &str {
        &self.msg
    }
}
