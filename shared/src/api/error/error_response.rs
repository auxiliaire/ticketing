use super::error_detail::ErrorDetail;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/*
{
    "code": "422 Unprocessable Entity",
    "details": {
        "errors": [
            "Passwords should match."
        ],
        "properties": {
            "password": {
                "errors": [
                    "Password should contain alphanumeric and special characters at a length range of 8-20."
                ]
            },
            "role": {
                "errors": [
                    "Role should be one of the followings: Developer, Manager."
                ]
            }
        }
    },
    "message": "Validation Error",
    "origin": "validation_rejection"
}
*/

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub details: Option<ErrorDetail>,
    pub message: String,
    pub origin: String,
}

impl From<String> for ErrorResponse {
    fn from(value: String) -> Self {
        Self {
            code: String::from(""),
            details: None,
            message: value,
            origin: String::from(""),
        }
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( code: '{}', message: '{}', origin: '{}' )",
            self.code, self.message, self.origin,
        )
    }
}
