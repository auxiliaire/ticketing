use self::{
    auth_backend::AuthBackend,
    config::MailConfig,
    consts::{ADMIN_EMAIL, CLIENT_URL, MAX_UPLOAD_LIMIT, SERVER_IP, SERVER_PORT},
    jwt::JwtLayer,
    services::notification_service::NotificationService,
    tasks::queue_mailer::QueueMailer,
};
use anyhow::Context;
use axum::{extract::DefaultBodyLimit, Extension, Router};
use axum_csrf::{CsrfConfig, CsrfLayer};
use axum_login::{
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use fang::AsyncQueue;
use fang::NoTls;
use fang::{AsyncQueueable, AsyncRunnable};
use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
    HeaderValue, Method,
};
use lettre::Message;
use redis::Client;
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub mod auth_backend;
pub mod auth_utils;
pub mod config;
pub mod consts;
pub mod error;
pub mod jwt;
pub mod login_controller;
pub mod query;
pub mod resources;
pub mod services;
pub mod tasks;
pub mod template_models;
pub mod validated_json;

pub async fn serve(
    store: Client,
    db: DatabaseConnection,
    queue: AsyncQueue<NoTls>,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&SocketAddr::new(SERVER_IP, *SERVER_PORT))
        .await
        .context("failed to bind listener");
    axum::serve(
        listener.unwrap(),
        router(store, db, queue).into_make_service(),
    )
    .await
    .context("failed to serve API")
}

pub fn router(store: Client, db: DatabaseConnection, queue: AsyncQueue<NoTls>) -> Router {
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

    let mail_config = MailConfig::default();
    let notification_service = NotificationService::new(mail_config);
    if let Err(e) = notification_service.send_email(
        Message::builder()
            .from("System <system@example.com>".parse().unwrap())
            .to(ADMIN_EMAIL.parse().unwrap())
            .subject("NotificationService initialized")
            .body("NotificationService has just been initialized.\nApplication was probably started/restarted.".to_owned())
            .unwrap(),
    ).context("Test email was not successful") {
        tracing::warn!("Unable to initialize Notification Service. Reason: {}", e);
    }

    let mut q = queue.clone();
    let task = QueueMailer {};
    tokio::spawn(async move {
        if let Err(e) = q.schedule_task(&task as &dyn AsyncRunnable).await {
            tracing::warn!("Unable to schedule task. Reason: {}", e);
        }
    });

    Router::new()
        .merge(resources::ticket_attachments_resource::router())
        .layer(DefaultBodyLimit::max(1024 * 1024 * (*MAX_UPLOAD_LIMIT)))
        .merge(resources::preferences_resource::router())
        .merge(resources::users_resource::router())
        .merge(resources::tickets_resource::router())
        .merge(resources::ticket_updates_resource::router())
        .merge(resources::comments_resource::router())
        .merge(resources::projects_resource::router())
        .layer(jwt_layer)
        .merge(login_controller::router())
        .layer(auth_layer)
        .layer(Extension(auth_backend))
        .layer(Extension(notification_service))
        .layer(Extension(queue))
        .layer(Extension(db))
        .layer(Extension(store))
        .layer(csrf_layer)
        .layer(cors_layer)
}
