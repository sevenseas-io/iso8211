use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ReadError {
    message: String,
}

impl ReadError {
    pub(crate) fn new(msg: &str) -> ReadError {
        ReadError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ReadError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<std::io::Error> for ReadError {
    fn from(error: std::io::Error) -> Self {
        ReadError {
            message: error.to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for ReadError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        ReadError {
            message: error.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for ReadError {
    fn from(error: std::num::ParseIntError) -> Self {
        ReadError {
            message: error.to_string(),
        }
    }
}

impl From<std::str::Utf8Error> for ReadError {
    fn from(error: std::str::Utf8Error) -> Self {
        ReadError {
            message: error.to_string(),
        }
    }
}
