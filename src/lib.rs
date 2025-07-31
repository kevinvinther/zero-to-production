use axum::{Router, http::StatusCode, routing::get, serve::Serve};
use std::net::TcpListener;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn run(
    listener: TcpListener,
) -> Result<Serve<tokio::net::TcpListener, Router, Router>, std::io::Error> {
    let app = Router::new().route("/health_check", get(health_check));

    listener.set_nonblocking(true)?;

    let listener = tokio::net::TcpListener::from_std(listener)?;

    let server = axum::serve(listener, app);

    Ok(server)
}
