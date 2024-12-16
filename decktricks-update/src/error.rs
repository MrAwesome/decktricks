use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub(crate) struct SimpleError(pub(crate) String);

impl Error for SimpleError {}

impl Display for SimpleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
