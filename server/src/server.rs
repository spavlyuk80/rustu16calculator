use crate::cli::ServerConfig;
use crate::calculator::Calculator;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::time::sleep;
use tokio::time::Duration;

pub(crate) async fn server_handler(config: &ServerConfig) {
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let app = Router::new().route("/", get(root));

    // check if port is in use
    loop {
        match TcpListener::bind(("127.0.0.1", config.port)).await {
            Ok(_) => break,
            Err(e) => {
                println!("got an error: {} will retry in 1 sec", e);
                let _ = sleep(Duration::from_secs(1)).await;
            },
        }
    }

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn root(query_params: Query<Calculator>) -> Result<String, (StatusCode, String)> {

    match query_params.get_result() {
        Ok(res) => Ok(format!("{}", res)),
        Err(e) => {
            let x = (
                StatusCode::UNPROCESSABLE_ENTITY, e.to_string()
            );
            Err(x)
        },
    }
}