use std::fmt::{Debug, Display};

#[derive(Debug)]
pub(crate) struct GdCastError<T: Debug>(T);

impl<T: Debug> std::error::Error for GdCastError<T> {}

impl<T: Debug> Display for GdCastError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub(crate) fn gderr<T: Debug + 'static>(inner: T) -> Box<dyn std::error::Error> {
    Box::new(GdCastError(inner))
}
