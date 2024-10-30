use std::{fmt::Display, error::Error};

#[derive(Debug)]
pub struct ParseError(pub usize, pub usize, pub &'static str);

impl Display for ParseError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "Unable to Read File Type"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}