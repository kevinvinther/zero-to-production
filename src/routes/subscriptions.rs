use crate::startup::AppState;
use axum::http::StatusCode;
use axum::{Extension, Form};
use serde::Deserialize;
use sqlx::types::chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(
    Extension(state): Extension<Arc<AppState>>,
    Form(form): Form<FormData>,
) -> StatusCode {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&state.pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
