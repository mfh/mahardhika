use std::error::Error;
use std::io::{self, Read, BufRead, BufReader, ErrorKind};
use std::fmt;
use regex::Regex;
use url::percent_encoding;

#[derive(debug)]
pub struct ParseError {
    description: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: & mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.description
    }
}

pub trait ParserHandler {
    fn on_method(&mut self, _method: & str) -> Result<(), ParseError> { Ok(()) }
}
