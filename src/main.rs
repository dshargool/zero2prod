use std::net::TcpListener;
use zero2prod::{startup::run, configuration::get_configuration};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read config.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener).await.unwrap();
}
