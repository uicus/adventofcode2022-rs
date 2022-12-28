use std::fmt;

#[derive(Debug)]
pub struct ParseInputError;

impl std::error::Error for ParseInputError {}

impl fmt::Display for ParseInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unparseable input!")
    }
}
