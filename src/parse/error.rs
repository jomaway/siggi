// Errors

use std::{fmt, error::Error};

#[derive(Debug, PartialEq)]
pub struct ParseWaveError {
    details: String,
}

impl ParseWaveError {
    pub fn new(msg: &str) -> ParseWaveError {
        ParseWaveError{details: msg.to_string()}
    }
}

impl fmt::Display for ParseWaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.details)
    }
}

impl Error for ParseWaveError {
    fn description(&self) -> &str {
        &self.details
    }
}