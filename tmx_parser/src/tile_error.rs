use std::num::ParseFloatError;
use std::str::ParseBoolError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum TMXError {
    Float(ParseFloatError),
    Bool(ParseBoolError),
    Int(ParseIntError),
}

impl From<ParseFloatError> for TMXError {
    fn from(t: ParseFloatError) -> Self {
        TMXError::Float(t)
    }
}

impl From<ParseBoolError> for TMXError {
    fn from(t: ParseBoolError) -> Self {
        TMXError::Bool(t)
    }
}

impl From<ParseIntError> for TMXError {
    fn from(t: ParseIntError) -> Self {
        TMXError::Int(t)
    }
}
