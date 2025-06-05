use std::error;
use std::fmt;

#[derive(Debug)]
pub enum OUError {
    Error1,
}

impl fmt::Display for OUError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OUError::Error1 => write!(f, "Oops")
        }
    }

}

impl error::Error for OUError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { 
        match *self {
            OUError::Error1 => None
        }
    }

}

pub type OUResult<T> = std::result::Result<T, OUError>;
