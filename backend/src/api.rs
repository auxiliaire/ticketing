use anyhow::Context;
use axum::{Extension, Router};
use http::Method;
use sea_orm::DatabaseConnection;
use shared::api::get_socket_address;
use tower_http::cors::{Any, CorsLayer};

pub mod error;
pub mod filters;
pub mod resources;
pub mod search;
pub mod validated_json;

pub async fn serve(db: DatabaseConnection) -> anyhow::Result<()> {
    axum::Server::bind(&get_socket_address())
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
        .merge(resources::users_resource::router())
        .merge(resources::tickets_resource::router())
        .merge(resources::ticket_updates_resource::router())
        .merge(resources::comments_resource::router())
        .merge(resources::projects_resource::router())
        .layer(Extension(db))
        .layer(cors)
}
