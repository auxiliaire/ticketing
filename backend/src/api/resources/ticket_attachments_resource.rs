use std::path::Path;

use axum::{routing::post, Extension, Json, Router};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use entity::users;
use http::StatusCode;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

use crate::api::error::ApiError;

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
    db: Extension<DatabaseConnection>,
    request_user: Extension<users::Model>,
    TypedMultipart(UploadForm { file }): TypedMultipart<UploadForm>,
) -> Result<Json<UploadResponse>, ApiError> {
    let file_name = file.metadata.file_name.unwrap_or(String::from("data.bin"));
    let path = Path::new("/tmp").join(file_name.clone());

    tracing::debug!("Upload file name: {}", file_name);
    tracing::debug!("Upload file path: {:?}", path);

    match file.contents.persist(path) {
        Ok(_) => Ok(Json(UploadResponse {})),
        Err(e) => Err(ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}
