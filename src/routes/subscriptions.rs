use serde::Deserialize;
use axum::extract::Form;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String
}
pub async fn subscribe(Form(_formdata): Form<FormData>) {
}
