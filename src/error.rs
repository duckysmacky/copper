#![allow(dead_code)]
use std::fmt::{Display, Formatter};
use std::process::Output;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CompilerError(String),
    CompileError(Output),
    LinkError(Output),
    BuildError(Output),
    EnumParseError(String),
    MissingConfigError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // TODO: add compile error output
            Error::CompilerError(e) => write!(f, "Compiler error: {}", e),
            Error::CompileError(output) => f.write_fmt(format_args!("Compiling failed ({})", output.status)),
            Error::LinkError(output) => f.write_fmt(format_args!("Linking failed ({})", output.status)),
            Error::BuildError(output) => f.write_fmt(format_args!("Build failed ({})", output.status)),
            Error::EnumParseError(s) => f.write_fmt(format_args!("Unable to parse an enum: {}", s)),
            Error::MissingConfigError(s) => f.write_fmt(format_args!("Missing a required configuration field: {}", s)),
        }
    }
}