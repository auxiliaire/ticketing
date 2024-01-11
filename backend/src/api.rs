use self::auth_backend::AuthBackend;
use anyhow::Context;
use axum::{Extension, Router};
use axum_login::{
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use http::Method;
use sea_orm::DatabaseConnection;
use shared::api::get_socket_address;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub mod auth_backend;
pub mod consts;
pub mod error;
pub mod login_controller;
pub mod query;
pub mod resources;
pub mod validated_json;

pub async fn serve(db: DatabaseConnection) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&get_socket_address())
        .await
        .context("failed to bind listener");
    axum::serve(listener.unwrap(), router(db).into_make_service())
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

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let auth_backend = AuthBackend::new(db.clone());
    let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

    Router::new()
        .merge(resources::users_resource::router())
        .merge(resources::tickets_resource::router())
        .merge(resources::ticket_updates_resource::router())
        .merge(resources::comments_resource::router())
        .merge(resources::projects_resource::router())
        .merge(login_controller::router())
        .layer(auth_layer)
        .layer(ServiceBuilder::new().layer(cors).layer(Extension(db)))
}
