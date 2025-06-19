use std::fmt::{Display, Formatter};
use std::io;
use std::process::Output;
use crate::error::parse_output;

pub type Result<T> = std::result::Result<T, Error>;

/// GCC-specific error types
#[derive(Debug)]
pub enum Error {
    /// Error related to the compilation of the source files
    CompileError(Output),
    /// Error related to the linking of the object files
    LinkError(Output),
    /// IO Error
    IOError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CompileError(o) => write!(f, "{}", parse_output(o)),
            Error::LinkError(o) => write!(f, "{}", parse_output(o)),
            Error::IOError(s) => write!(f, "IO Error ({})", s),
        }   
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err.to_string())
    }
}