use self::{auth_backend::AuthBackend, consts::CLIENT_URL, jwt::JwtLayer};
use anyhow::Context;
use axum::{Extension, Router};
use axum_csrf::{CsrfConfig, CsrfLayer};
use axum_login::{
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
    HeaderValue, Method,
};
use redis::Client;
use sea_orm::DatabaseConnection;
use shared::api::get_socket_address;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub mod auth_backend;
pub mod auth_utils;
pub mod consts;
pub mod error;
pub mod jwt;
pub mod login_controller;
pub mod query;
pub mod resources;
pub mod validated_json;

pub async fn serve(store: Client, db: DatabaseConnection) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&get_socket_address())
        .await
        .context("failed to bind listener");
    axum::serve(listener.unwrap(), router(store, db).into_make_service())
        .await
        .context("failed to serve API")
}

pub fn router(store: Client, db: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::POST,
            Method::GET,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN])
        .allow_origin(CLIENT_URL.parse::<HeaderValue>().unwrap());
    let cors_layer = ServiceBuilder::new().layer(cors);

    let csrf = CsrfConfig::default();
    let csrf_layer = CsrfLayer::new(csrf);

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let auth_backend = AuthBackend::new(db.clone());
    let auth_layer = AuthManagerLayerBuilder::new(auth_backend.clone(), session_layer).build();

    let jwt_layer = JwtLayer::default();

    Router::new()
        .merge(resources::users_resource::router())
        .merge(resources::tickets_resource::router())
        .merge(resources::ticket_updates_resource::router())
        .merge(resources::comments_resource::router())
        .merge(resources::projects_resource::router())
        .layer(jwt_layer)
        .merge(login_controller::router())
        .layer(auth_layer)
        .layer(Extension(auth_backend))
        .layer(Extension(db))
        .layer(Extension(store))
        .layer(csrf_layer)
        .layer(cors_layer)
}
