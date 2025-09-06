use std::{fmt};
use std::process::Output;

/// A trait for error kinds used in CopperError
/// 
/// This trait requires the implementation of the Display trait for better error messages
/// when printing the error kind
pub trait CopperErrorKind: fmt::Display {}

/// A generic error type for Copper-related errors that can hold different kinds of error 
/// information. The error kind must implement the CopperErrorKind trait. This allows for
/// more specific error handling while still using a common error structure.
/// 
/// This error type should be used directly, instead the K generic type parameter should be
/// used to specify the exact kind of error being represented by each of the modules.
#[derive(Debug)]
pub struct CopperError<T: CopperErrorKind>
{
    kind: T,
    cause: Option<String>,
}

impl<T: CopperErrorKind> CopperError<T> {
    /// Creates a new CopperError with the specified kind and cause
    pub fn new(kind: T, cause: impl fmt::Display) -> Self {
        CopperError { kind, cause: Some(cause.to_string()) }
    }
    
    /// Creates a new CopperError with the specified kind and no cause
    pub fn no_cause(kind: T) -> Self {
        CopperError { kind, cause: None }
    }

    /// Returns a reference to the error message
    pub fn message(&self) -> String {
        self.kind.to_string()
    }
 
    /// Returns a reference to the error kind
    pub fn kind(&self) -> &T {
        &self.kind
    }
    
    /// Returns an optional reference to the cause of the error, if any was provided
    pub fn cause(&self) -> Option<&String> {
        self.cause.as_ref()
    }
}

impl<T: CopperErrorKind> fmt::Display for CopperError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;
        if let Some(cause) = &self.cause {
            write!(f, "\n\tCause: {}", cause)
        } else {
            Ok(())
        }
    }
}

/// Parses the output object and returns a formatted string containing exit code, stdout and stderr
pub fn parse_output(output: &Output) -> String {
    let mut message = String::new();
    message.push_str(format!("{}", output.status).as_str());

    if output.stdout.len() > 0 {
        message.push_str(format!("\nStdout:\n{}", String::from_utf8_lossy(&output.stdout)).as_str());
    }

    if output.stderr.len() > 0 {
        message.push_str(format!("\nStderr:\n{}", String::from_utf8_lossy(&output.stderr)).as_str());
    }

    message
}
