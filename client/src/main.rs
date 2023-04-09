mod api;
mod client_error;
mod expression_handler;
mod structures;

use crate::expression_handler::evaluate_expression;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut stdin_reader = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    loop {
        // cli reader
        println!("Enter an expression in the format e.g. 14 - 12");
        tokio::io::stdout().flush().await.unwrap();
        let input = match stdin_reader.next_line().await {
            Ok(Some(line)) => line,
            _ => break,
        };

        let _ = match evaluate_expression(input.as_str()).await {
            Ok(result) => println!("{}", result),
            Err(e) => println!("{}", e),
        };
        println!("\n");
    }
}
