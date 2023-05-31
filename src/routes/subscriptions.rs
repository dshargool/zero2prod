use axum::extract::{Extension, Form};
use chrono::Utc;
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
pub async fn subscribe(
    Extension(pool): Extension<PgPool>,
    Form(formdata): Form<FormData>,
) -> hyper::StatusCode {
    tracing::info!("Saving new subscriber details to database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        formdata.email,
        formdata.name,
        Utc::now()
    )
    .execute(&pool)
    .await
    {
    Ok(_) => {
        tracing::info!("New subscriber details have been saved");
        StatusCode::OK
    }
    Err(e) => {
        tracing::error!("Failed to execute query: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    }
    }
}
