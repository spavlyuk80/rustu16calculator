use crate::client_error::ClientError;
use crate::expression::Expression;
use url::form_urlencoded;
use crate::cli::ServerConfig;

pub async fn make_request(expression: &Expression, config: &ServerConfig) -> Result<String, ClientError> {

    let base_url = format!("http://127.0.0.1:{}?", config.port);
    // encode url safe
    let url = form_urlencoded::Serializer::new(base_url.to_string())
        .append_pair("a", expression.a.to_string().as_str())
        .append_pair("b", expression.b.to_string().as_str())
        .append_pair("sign", expression.sign.as_str())
        .finish();
    let response = match reqwest::get(url).await {
        Ok(value) => value,
        Err(_) => {
            return Err(ClientError {
                message: "There was an error in calling the server".to_string(),
            })
        }
    };
    if response.status() != 200 {
        return Err(ClientError {
            message: format!("Error type of {}", response.text().await.unwrap()).to_string(),
        });
    }
    let res = match response.text().await {
        Ok(res) => res,
        Err(_) => {
            return Err(ClientError {
                message: "Could not parse response text".to_string(),
            })
        }
    };
    Ok(res)
}
