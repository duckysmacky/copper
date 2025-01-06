#![allow(dead_code)]
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EnumParseError(String),
    MissingConfigError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EnumParseError(s) => f.write_fmt(format_args!("Unable to parse an enum: {}", s)),
            Error::MissingConfigError(s) => f.write_fmt(format_args!("Missing a required configuration field: {}", s)),
        }
    }
}