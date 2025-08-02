use axum::Form;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(Form(_form): Form<FormData>) -> &'static str {
    ""
}
