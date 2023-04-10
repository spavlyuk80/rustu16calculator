use crate::server_error::ServerError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Calculator {
    a: u16,
    b: u16,
    sign: String,
}

impl Calculator {
    // u16 can overflow and underflow. instead of raising en error
    // it makes sense to convert u16 to the types we can properly handle
    // and return number. I could do checked_* operations, but why?
    pub(crate) fn get_result(&self) -> Result<String, ServerError> {
        let result: String = match self.sign.as_str() {
            // can overflow, safe to do with u32
            "+" => {
                let u32sum = self.a as u32 + self.b as u32;
                let output = u32sum.to_string();
                output
            },
            // can underflow
            "-" => {
                let result = self.a as i32 - self.b as i32;
                result.to_string()
            }
            // can overflow, u64 is safe
            "*" => {
                let result = self.a as u64 * self.b as u64;
                result.to_string()
            },
            "/" => {
                if self.b == 0 {
                    return Err(ServerError {
                        message: "Division by zero".to_string(),
                    });
                }
                let result = self.a as f32/ self.b as f32;
                result.to_string()
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
