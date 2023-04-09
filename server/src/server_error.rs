use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ServerError {
    pub(crate) message: String,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServerError: {}", self.message)
    }
}

impl Error for ServerError {}
