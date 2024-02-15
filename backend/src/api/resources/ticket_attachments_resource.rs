use crate::api::{
    consts::{ADMIN_EMAIL, BUCKET_NAME},
    error::ApiError,
    services::notification_service::NotificationService,
};
use anyhow::Context;
use askama_axum::IntoResponse;
use axum::{
    extract::Path,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::WithRejection;
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use entity::{
    ticket_attachments::{self, Model},
    users,
};
use http::StatusCode;
use lettre::Message;
use object_store::{
    aws::{AmazonS3, AmazonS3Builder},
    ObjectStore,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use std::path;
use tempfile::NamedTempFile;
use tokio::io::AsyncWriteExt;

pub fn router() -> Router {
    Router::new()
        .route("/tickets/:id/attachments", post(upload_file))
        .route("/tickets/:id/attachments/:filename", get(download_file))
}

#[derive(TryFromMultipart)]
pub struct UploadForm {
    #[form_data(limit = "5MiB")]
    pub file: FieldData<NamedTempFile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadResponse {}

#[axum::debug_handler]
pub async fn upload_file(
    db: Extension<DatabaseConnection>,
    _request_user: Extension<users::Model>,
    Extension(notification_service): Extension<NotificationService>,
    WithRejection(Path(ticket_id), _): WithRejection<Path<u64>, ApiError>,
    TypedMultipart(UploadForm { file }): TypedMultipart<UploadForm>,
) -> Result<Json<UploadResponse>, ApiError> {
    let file_name = file.metadata.file_name.unwrap_or(String::from("data.bin"));
    let path = path::Path::new("/tmp").join(file_name.clone());

    match file.contents.persist(path.clone()) {
        Ok(_) => {
            tracing::debug!("Uploading file succeeded to: {:?}", path);
            tokio::spawn(async move {
                let bucket: AmazonS3 = AmazonS3Builder::from_env()
                    .with_bucket_name(BUCKET_NAME.clone())
                    .build()
                    .context("Amazon bucket could not be built")
                    .unwrap();
                let raw_path = format!("tickets/{}/attachments/{}", ticket_id, file_name);
                let obj_path = object_store::path::Path::from(raw_path.clone());
                let (_multipart_id, mut writer) = bucket
                    .put_multipart(&obj_path)
                    .await
                    .context("Multipart upload failed")
                    .unwrap();
                let bytes = tokio::fs::read(path.clone()).await.unwrap();
                writer.write_all(&bytes).await.unwrap();
                writer.flush().await.unwrap();
                writer.shutdown().await.unwrap();
                tracing::debug!(
                    "Object successfully uploaded to store. Path: {}",
                    raw_path.clone()
                );

                let local_path = path.to_str().map(String::from);
                let attachment = ticket_attachments::ActiveModel {
                    ticket_id: Set(ticket_id),
                    local_path: Set(local_path),
                    path: Set(raw_path.clone()),
                    ..Default::default()
                }
                .insert(&*db)
                .await;

                match attachment {
                    Ok(Model { id, .. }) => {
                        tracing::debug!("Attachment saved successfully at {}", id)
                    }
                    Err(e) => {
                        tracing::error!("Failed to save attachment. Error: '{}'", e.to_string())
                    }
                }

                let _ = send_upload_notification(notification_service, raw_path);
            });
            Ok(Json(UploadResponse {}))
        }
        Err(e) => Err(ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}

#[axum::debug_handler]
pub async fn download_file(
    WithRejection(Path((ticket_id, file_name)), _): WithRejection<Path<(u64, String)>, ApiError>,
) -> Result<impl IntoResponse, ApiError> {
    let bucket: AmazonS3 = AmazonS3Builder::from_env()
        .with_bucket_name(BUCKET_NAME.clone())
        .build()
        .context("Amazon bucket could not be built")
        .unwrap();
    let raw_path = format!("tickets/{}/attachments/{}", ticket_id, file_name);
    let obj_path = object_store::path::Path::from(raw_path.clone());

    let stream = bucket
        .get(&obj_path)
        .await
        .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .into_stream();

    Ok(axum::body::Body::from_stream(stream))
}

fn send_upload_notification(
    notification_service: NotificationService,
    path: String,
) -> Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error> {
    notification_service.send_email(
        Message::builder()
            .from("System <system@example.com>".parse().unwrap())
            .to(ADMIN_EMAIL.to_string().parse().unwrap())
            .subject("Upload notification")
            .body(format!(
                "A new resource was uploaded to storage.\nPath: {}",
                path
            ))
            .unwrap(),
    )
}
