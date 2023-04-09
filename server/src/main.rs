mod calculator;
mod cli;
mod server;
mod server_error;

use crate::cli::ServerConfig;
use crate::server::server_handler;
use clap::Parser;
use cli::Cli;

#[cfg(test)]
include!("tests/test_calc.rs");

#[tokio::main]
async fn main() {
    let matches = Cli::parse();
    let config: ServerConfig = ServerConfig::new_from_file_config(matches.config);

    let server_thread = tokio::spawn(async move {
        server_handler(&config).await;
    });

    // Wait for the server thread to complete
    server_thread.await.unwrap();
}
