use std::fmt::Display;

use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
};
use sea_orm::{strum::Display, DbErr};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub struct JsonError {
    #[serde(skip_serializing)]
    status: StatusCode,
    code: Option<String>,
    message: String,
    origin: Option<String>,
}

impl Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(status = {}, message = '{}')",
            self.status, self.message,
        )
    }
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
    DbAppError(#[from] DbErr),
    HandlerError(#[from] JsonError),
}

impl ApiError {
    pub fn new(status_code: StatusCode, message: String) -> Self {
        ApiError::HandlerError(JsonError::from((status_code, message)))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::JsonExtractorRejection(json_rejection) => JsonError::from((
                json_rejection.status(),
                json_rejection.body_text(),
                String::from("json_rejection"),
            )),
            ApiError::PathExtractorRejection(path_rejection) => JsonError::from((
                path_rejection.status(),
                path_rejection.body_text(),
                String::from("path_rejection"),
            )),
            ApiError::DbAppError(db_error) => JsonError::from((
                StatusCode::INTERNAL_SERVER_ERROR,
                db_error.to_string(),
                String::from("db_error"),
            )),
            ApiError::HandlerError(handler_error) => handler_error,
        }
        .into_response()
    }
}
