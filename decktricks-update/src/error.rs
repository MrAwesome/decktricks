use std::{error::Error, fmt::Display};

#[derive(Debug)]
struct SimpleError(String);

impl Error for SimpleError {}

impl Display for SimpleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) fn dt_err(s: String) -> Box<dyn Error> {
    Box::new(SimpleError(s))
}
