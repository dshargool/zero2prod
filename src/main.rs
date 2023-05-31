use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let configuration = get_configuration().expect("Failed to read config.");

    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Could not connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    let server = run(listener, connection).await.unwrap();
    server.await.unwrap();
}
