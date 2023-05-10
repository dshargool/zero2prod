use axum::routing::{get, post, IntoMakeService};
use axum::{Router, Server};
use hyper::server::conn::AddrIncoming;
use std::net::TcpListener;
use crate::routes::subscribe;
use crate::routes::health_check;

pub async fn run(
    listener: TcpListener,
) -> hyper::Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    println!("Hello, world!");

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
