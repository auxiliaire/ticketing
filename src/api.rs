use anyhow::Context;
use axum::{Extension, Router};
use http::Method;
use sqlx::MySqlPool;
use tower_http::cors::{Any, CorsLayer};

pub mod error;
pub mod tickets;

pub async fn serve(db: MySqlPool) -> anyhow::Result<()> {
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(router(db).into_make_service())
        .await
        .context("failed to serve API")
}

pub fn router(db: MySqlPool) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        .merge(tickets::router())
        .layer(Extension(db))
        .layer(cors)
}
