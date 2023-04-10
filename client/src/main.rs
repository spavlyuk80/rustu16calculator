mod api;
mod client_error;
mod expression_handler;
mod expression;
mod cli;

use clap::Parser;
use crate::cli::ServerConfig;
use crate::expression_handler::evaluate_expression;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use cli::Cli;

#[tokio::main]
async fn main() {
    let matches = Cli::parse();
    let config: ServerConfig = ServerConfig::new_from_file_config(matches.config);

    let mut stdin_reader = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    loop {
        println!("Enter an expression in the format e.g. 14 - 12");
        tokio::io::stdout().flush().await.unwrap();
        let input = match stdin_reader.next_line().await {
            Ok(Some(line)) => line,
            _ => break,
        };

        let _ = match evaluate_expression(input.as_str(), &config).await {
            Ok(result) => println!("{}", result),
            Err(e) => println!("{}", e),
        };
        println!("\n");
    }
}
