use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
};
use sea_orm::{strum::Display, DbErr};
use serde::Serialize;
use serde_json::json;
use serde_valid::validation::Errors;
use shared::api::error::{error_detail::ErrorDetail, error_response::ErrorResponse};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub struct JsonError {
    #[serde(skip_serializing)]
    status: StatusCode,
    code: Option<String>,
    message: String,
    origin: Option<String>,
    details: Option<Errors>,
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
            details: Option::None,
        }
    }
}

impl From<(StatusCode, String, Option<String>)> for JsonError {
    fn from((status, message, code): (StatusCode, String, Option<String>)) -> Self {
        JsonError {
            status,
            code,
            message,
            origin: Option::None,
            details: Option::None,
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
            details: Option::None,
        }
    }
}

impl From<(StatusCode, String, String, Errors)> for JsonError {
    fn from((status, message, origin, details): (StatusCode, String, String, Errors)) -> Self {
        JsonError {
            status,
            code: Option::None,
            message,
            origin: Option::Some(origin),
            details: Option::Some(details),
        }
    }
}

impl From<AuthError> for JsonError {
    fn from(
        AuthError {
            status,
            message,
            code,
        }: AuthError,
    ) -> Self {
        JsonError {
            status,
            code,
            message,
            origin: Option::None,
            details: Option::None,
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
            json!(ErrorResponse {
                code: self.code.unwrap_or(self.status.to_string()),
                message: self.message,
                origin: self.origin.unwrap_or(String::from("unspecified")),
                details: self.details.map(ErrorDetail::from),
            })
            .to_string(),
        )
            .into_response()
    }
}

#[derive(Debug, Error, Serialize)]
pub struct AuthError {
    #[serde(skip_serializing)]
    pub(crate) status: StatusCode,
    pub code: Option<String>,
    pub message: String,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(status = {}, message = '{}', code = '{}')",
            self.status,
            self.message,
            <std::option::Option<std::string::String> as Clone>::clone(&self.code)
                .unwrap_or_default()
        )
    }
}

impl Default for AuthError {
    fn default() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: Default::default(),
            message: String::from("Authentication/Authorization Error"),
        }
    }
}

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    PathExtractorRejection(#[from] PathRejection),
    ValidationRejection(#[from] Errors),
    DbAppError(#[from] DbErr),
    HandlerError(#[from] JsonError),
    AuthError(#[from] AuthError),
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
            ApiError::ValidationRejection(validation_errors) => JsonError::from((
                StatusCode::UNPROCESSABLE_ENTITY,
                String::from("Validation Error"),
                String::from("validation_rejection"),
                validation_errors,
            )),
            ApiError::DbAppError(db_error) => JsonError::from((
                StatusCode::INTERNAL_SERVER_ERROR,
                db_error.to_string(),
                String::from("db_error"),
            )),
            ApiError::HandlerError(handler_error) => handler_error,
            ApiError::AuthError(auth_error) => JsonError::from(auth_error),
        }
        .into_response()
    }
}
