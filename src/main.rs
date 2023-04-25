use axum::{response::Html, routing::get, Router, extract::Path};

async fn root() -> String {
    format!("Hiya Guy")
}

async fn greet(Path(user_name): Path<String>) -> String {
    format!("Hello {}", &user_name)
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new().route("/", get(root))
        .route("/*user_name", get(greet));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}
