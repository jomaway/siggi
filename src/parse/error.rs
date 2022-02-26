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


#[derive(Debug)]
pub enum ParseError {
    ParseWaveError(ParseWaveError),
    ParseSignalError,
    ParseConfigError,
    ParseDiagramError,
    ParseJsonError(serde_json::Error)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::ParseWaveError(_) => write!(f," Error occured during wave parsing"),
            ParseError::ParseSignalError => write!(f," Error occured during signal parsing"),
            ParseError::ParseConfigError => write!(f," Error occured during config parsing"),
            ParseError::ParseDiagramError => write!(f," Error occured during diagram parsing"),
            ParseError::ParseJsonError(_) => write!(f," Error occured during json parsing"),
        }
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(err: serde_json::Error) -> Self {
        ParseError::ParseJsonError(err)
    }
}

impl From<ParseWaveError> for ParseError {
    fn from(err: ParseWaveError) -> Self {
        ParseError::ParseWaveError(err)
    }
}