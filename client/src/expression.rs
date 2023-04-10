use crate::client_error::ClientError;
use regex::Captures;

#[derive(Debug)]
pub struct Expression {
    pub(crate) a: u16,
    pub(crate) b: u16,
    pub(crate) sign: String,
}

impl Expression {
    pub(crate) fn new(captures: &Captures) -> Result<Expression, ClientError> {
        let a = match captures.get(1) {
            None => {
                return Err(ClientError {
                    message: "Could not parse 'a'".to_string(),
                })
            }
            Some(result) => result.as_str().parse::<u16>().unwrap(),
        };
        let b: u16 = match captures.get(3) {
            None => {
                return Err(ClientError {
                    message: "Could not parse 'b'".to_string(),
                })
            }
            Some(result) => result.as_str().parse::<u16>().unwrap(),
        };
        let sign = match captures.get(2) {
            None => {
                return Err(ClientError {
                    message: "Could not parse 'sign'".to_string(),
                })
            }
            Some(result) => result.as_str().to_string(),
        };
        Ok(Expression { a, b, sign })
    }
}
