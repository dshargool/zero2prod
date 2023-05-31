use crate::routes;
use axum::extract::Extension;
use axum::routing::{get, post, IntoMakeService};
use axum::{Router, Server};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use std::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub async fn run(
    listener: TcpListener,
    pool: PgPool,
) -> hyper::Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
        .layer(Extension(pool))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new()
                    .level(Level::INFO)),
        );

    tracing::info!("Listening on {:?}", listener);

    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
