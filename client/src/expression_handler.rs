use crate::api::make_request;
use crate::client_error::ClientError;
use crate::expression::Expression;
use regex::Regex;
use crate::cli::ServerConfig;

pub async fn evaluate_expression(input: &str, config: &ServerConfig) -> Result<String, ClientError> {
    let re = Regex::new(r"^\s*(\d+)\s*([+\-*/])\s*(\d+)\s*$").unwrap();

    if !input
        .trim()
        .chars()
        .all(|c| c.is_digit(10) || c == ' ' || "+-/*".contains(c))
    {
        return Err(ClientError {
            message: "Expression can contain only [0-9] and signs: + - / *".to_string(),
        });
    }

    let caps = match re.captures(input) {
        Some(caps) => caps,
        None => {
            return Err(ClientError {
                message: "Could not split the expression string into 'a sign b'".to_string(),
            })
        }
    };

    let expression = match Expression::new(&caps) {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    let response = match make_request(&expression, &config).await {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    let response_to_print = format!(
        "{} {} {} = {}",
        expression.a, expression.sign, expression.b, response
    );
    Ok(response_to_print)
}
