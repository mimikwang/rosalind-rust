//! Custom errors for the crate
pub type Result<T> = std::result::Result<T, Error>;

/// The kind of error
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    IO,
    User,
}

/// Custom error type for the crate
#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "kind: {:?}, message: {}", self.kind, self.message)
    }
}

impl Error {
    /// Basic constructor for Error
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Error {
            kind,
            message: message.into(),
        }
    }

    /// Returns the kind of error
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::new(ErrorKind::IO, &e.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::new(ErrorKind::IO, &e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::new(ErrorKind::IO, &e.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::new(ErrorKind::IO, &e.to_string())
    }
}
