use anyhow::Context;
use axum::{Extension, Router};
use http::Method;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

pub mod error;
pub mod tickets;
pub mod users;

pub async fn serve(db: DatabaseConnection) -> anyhow::Result<()> {
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(router(db).into_make_service())
        .await
        .context("failed to serve API")
}

pub fn router(db: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::POST,
            Method::GET,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        .merge(users::router())
        .merge(tickets::router())
        .layer(Extension(db))
        .layer(cors)
}
