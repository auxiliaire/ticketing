use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
};
use sea_orm::strum::Display;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Serialize)]
pub struct JsonError {
    #[serde(skip_serializing)]
    status: StatusCode,
    code: Option<String>,
    message: String,
    origin: Option<String>,
}

impl From<(StatusCode, String)> for JsonError {
    fn from((status, message): (StatusCode, String)) -> Self {
        JsonError {
            status,
            code: Option::None,
            message,
            origin: Option::None,
        }
    }
}

impl From<(StatusCode, String, String)> for JsonError {
    fn from((status, message, origin): (StatusCode, String, String)) -> Self {
        JsonError {
            status,
            code: Option::None,
            message,
            origin: Option::Some(origin),
        }
    }
}

impl IntoResponse for JsonError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
            )],
            json!({
                "code": self.code.unwrap_or(self.status.to_string()),
                "message": self.message,
                "origin": self.origin.unwrap_or(String::from("unspecified")),
            })
            .to_string(),
        )
            .into_response()
    }
}

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    PathExtractorRejection(#[from] PathRejection),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::JsonExtractorRejection(json_rejection) => {
                (json_rejection.status(), json_rejection.body_text())
            }
            ApiError::PathExtractorRejection(path_rejection) => {
                (path_rejection.status(), path_rejection.body_text())
            }
        };

        JsonError::from((status, message, String::from("with_rejection"))).into_response()
    }
}
