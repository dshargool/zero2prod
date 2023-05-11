use crate::routes;
use axum::extract::Extension;
use axum::routing::{get, post, IntoMakeService};
use axum::{Router, Server};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use std::net::TcpListener;

pub async fn run(
    listener: TcpListener,
    pool: PgPool,
) -> hyper::Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    println!("Hello, world!");

    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
        .layer(Extension(pool));

    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
