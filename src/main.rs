use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read config.");

    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Could not connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener, connection).await.unwrap();
}
