use std::fmt::{Display, Formatter, Debug};
use std::error::Error;

#[derive(Debug)]
pub struct ResponseError<E> {
    code: u32,
    message: String,
    data: Option<E>
}

impl<E> Display for ResponseError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}; error code {}", self.message, self.code)
    }
}

impl<E: Debug> Error for ResponseError<E> {}
