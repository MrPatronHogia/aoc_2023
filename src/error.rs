#[derive(Debug)]
pub enum Error {
    Generic(String),
}
use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Generic(err) => write!(f, "Generic error: {}", err),
        }
    }
}
