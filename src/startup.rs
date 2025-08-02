use crate::routes::*;
use axum::routing::{get, post};
use axum::serve::Serve;
use axum::{Extension, Router};
use sqlx::PgPool;
use std::net::TcpListener;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn run(
    listener: TcpListener,
    pool: PgPool,
) -> Result<Serve<tokio::net::TcpListener, Router, Router>, std::io::Error> {
    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(Extension(state));

    listener.set_nonblocking(true)?;

    let listener = tokio::net::TcpListener::from_std(listener)?;

    let server = axum::serve(listener, app);

    Ok(server)
}
