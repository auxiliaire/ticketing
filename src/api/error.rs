use axum::{
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct JsonError {
    code: String,
    message: String,
}

pub fn to_uniform_response(code: StatusCode, message: String) -> impl IntoResponse {
    (
        code,
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        )],
        json!(JsonError {
            code: code.to_string(),
            message,
        })
        .to_string(),
    )
}
