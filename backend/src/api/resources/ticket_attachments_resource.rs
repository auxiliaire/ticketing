use anyhow::Context;
use axum::{routing::post, Extension, Json, Router};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use entity::users;
use http::StatusCode;
use object_store::{
    aws::{AmazonS3, AmazonS3Builder},
    ObjectStore,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tempfile::NamedTempFile;
use tokio::io::AsyncWriteExt;

use crate::api::{consts::BUCKET_NAME, error::ApiError};

pub fn router() -> Router {
    Router::new().route("/ticket_attachments", post(upload_file))
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
    _db: Extension<DatabaseConnection>,
    _request_user: Extension<users::Model>,
    TypedMultipart(UploadForm { file }): TypedMultipart<UploadForm>,
) -> Result<Json<UploadResponse>, ApiError> {
    let file_name = file.metadata.file_name.unwrap_or(String::from("data.bin"));
    let path = Path::new("/tmp").join(file_name.clone());

    match file.contents.persist(path.clone()) {
        Ok(_) => {
            tracing::debug!("Uploading file succeeded to: {:?}", path);
            tokio::spawn(async move {
                let bucket: AmazonS3 = AmazonS3Builder::from_env()
                    .with_bucket_name(BUCKET_NAME.clone())
                    .build()
                    .context("Amazon bucket could not be built")
                    .unwrap();
                let obj_path = object_store::path::Path::from(format!("attachment/{}", file_name));
                let (id, mut writer) = bucket
                    .put_multipart(&obj_path)
                    .await
                    .context("Multipart upload failed")
                    .unwrap();
                let bytes = tokio::fs::read(path).await.unwrap();
                writer.write_all(&bytes).await.unwrap();
                writer.flush().await.unwrap();
                writer.shutdown().await.unwrap();
                tracing::debug!("Object successfully uploaded to store. Id: {}", id);
                id
            });
            Ok(Json(UploadResponse {}))
        }
        Err(e) => Err(ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}
