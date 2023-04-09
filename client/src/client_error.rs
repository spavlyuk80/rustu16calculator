use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ClientError {
    pub(crate) message: String,
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientError: {}", self.message)
    }
}

impl Error for ClientError {}
