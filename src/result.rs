use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    s: String,
}

impl Error {
    pub fn new(s: String) -> Error {
        Error { s: s }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.s.as_str()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}
