#![allow(dead_code)]
use std::fmt::{Display, Formatter};
use std::process::Output;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IOError(String),
    CompilerError(String),
    CompileError(Output),
    LinkError(Output),
    BuildError(Output),
    EnumParseError(String),
    MissingConfigError(String),
}

// TODO: add better output for the Output
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(s) => write!(f, "IO error: {}", s),
            Error::CompilerError(s) => write!(f, "Compiler error: {}", s),
            Error::CompileError(output) => write!(f, "Compiling failed ({})", output.status),
            Error::LinkError(output) => write!(f, "Linking failed ({})", output.status),
            Error::BuildError(output) => write!(f, "Build failed ({})", output.status),
            Error::EnumParseError(s) => write!(f, "Unable to parse an enum: {}", s),
            Error::MissingConfigError(s) => write!(f, "Missing a required configuration field: {}", s),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error.to_string())
    }
}