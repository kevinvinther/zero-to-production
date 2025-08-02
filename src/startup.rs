use crate::routes::*;
use axum::Router;
use axum::routing::{get, post};
use axum::serve::Serve;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
) -> Result<Serve<tokio::net::TcpListener, Router, Router>, std::io::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    listener.set_nonblocking(true)?;

    let listener = tokio::net::TcpListener::from_std(listener)?;

    let server = axum::serve(listener, app);

    Ok(server)
}
