use std::error::Error;
use std::fmt;

// type Result<T> = std::result::Result<T, EccoSyntaxError>;

// https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html
#[derive(Debug)]
pub struct EccoSyntaxError {
    details: String,
}

impl EccoSyntaxError {
    pub fn new(msg: String) -> EccoSyntaxError {
        EccoSyntaxError { details: msg }
    }
}

impl fmt::Display for EccoSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax error: {}", self.details)
    }
}

impl Error for EccoSyntaxError {
    fn description(&self) -> &str {
        &self.details
    }
}
