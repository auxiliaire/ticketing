use anyhow::Context;
use axum::{Extension, Router};
use http::Method;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

pub mod comments_resource;
pub mod error;
pub mod projects_resource;
pub mod ticket_updates_resource;
pub mod tickets_resource;
pub mod users_resource;

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
        .merge(users_resource::router())
        .merge(tickets_resource::router())
        .merge(ticket_updates_resource::router())
        .merge(comments_resource::router())
        .merge(projects_resource::router())
        .layer(Extension(db))
        .layer(cors)
}
