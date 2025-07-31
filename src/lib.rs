use axum::{Router, http::StatusCode, routing::get};
use std::net::SocketAddr;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn run() {
    let app = Router::new().route("/health_check", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
