use std::io::Error;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ReadError {
    ParseError(String),
    IoError(Error),
    Utf8Error(FromUtf8Error),
    IntError(ParseIntError),
}

impl From<Error> for ReadError {
    fn from(error: Error) -> Self {
        ReadError::IoError(error)
    }
}

impl From<String> for ReadError {
    fn from(error: String) -> Self {
        ReadError::ParseError(error)
    }
}

impl From<FromUtf8Error> for ReadError {
    fn from(error: FromUtf8Error) -> Self {
        ReadError::Utf8Error(error)
    }
}

impl From<ParseIntError> for ReadError {
    fn from(error: ParseIntError) -> Self {
        ReadError::IntError(error)
    }
}

// impl From<std::str::Utf8Error> for ReadError {
//     fn from(error: std::str::Utf8Error) -> Self {
//         ReadError {
//             message: error.to_string(),
//         }
//     }
// }
