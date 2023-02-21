use std::{fmt, io};

#[derive(Debug)]
pub enum WanitError {
    HttpError,
    IoError,
}
impl std::error::Error for WanitError {}

impl fmt::Display for WanitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WanitError::HttpError => write!(f, "HTTP Error"),
            WanitError::IoError => write!(f, "IO Error"),
        }
    }
}
impl From<reqwest::Error> for WanitError {
    fn from(_: reqwest::Error) -> Self {
        WanitError::HttpError
    }
}

impl From<io::Error> for WanitError {
    fn from(_: io::Error) -> Self {
        WanitError::IoError
    }
}
