use crate::server_error::ServerError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Calculator {
    a: u16,
    b: u16,
    sign: String,
}

impl Calculator {
    pub(crate) fn get_result(&self) -> Result<String, ServerError> {
        let result: String = match self.sign.as_str() {
            "+" => {
                let result = self.a + self.b;
                result.to_string()
            },
            "-" => {
                if self.b > self.a {
                    return Err(ServerError {
                        message: "Integer underflow for type u16".to_string(),
                    });
                }
                let result = self.a - self.b;
                result.to_string()
            }
            "*" => {
                let result = self.a * self.b;
                result.to_string()
            },
            "/" => {
                if self.b == 0 {
                    return Err(ServerError {
                        message: "Division by zero".to_string(),
                    });
                }

                let result = self.a as f32/ self.b as f32;
                let integer_part = result.trunc() as u16;
                let decimal_part = (result.fract() * 100.0).round() as u16;
                format!("{}.{:02}", integer_part, decimal_part)

            }
            _ => {
                return Err(ServerError {
                    message: format!("Invalid sign: {}", self.sign).to_string(),
                })
            }
        };
        println!("{} {} {} = {}", self.a, self.sign, self.b, result);
        Ok(result)
    }
}
