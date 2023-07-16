use std::{collections::HashMap, fmt::Display, sync::Arc};

use implicit_clone::sync::{IArray, IString};
use serde::{Deserialize, Serialize};
use serde_valid::{
    json::{json, Value},
    validation::Errors,
};

use crate::validation::error_messages::{ErrorMessageList, ErrorMessages, ErrorsTrait};

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
pub struct PropertyError {
    pub errors: Vec<String>,
}

impl From<Vec<String>> for PropertyError {
    fn from(value: Vec<String>) -> Self {
        Self { errors: value }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorDetail {
    pub errors: Option<Vec<String>>,
    pub properties: Option<HashMap<String, PropertyError>>,
}

impl ErrorsTrait for ErrorDetail {
    fn get_common_messages(&self) -> ErrorMessages {
        self.errors.as_ref().map(|v| {
            IArray::<IString>::Rc(
                v.iter()
                    .map(|s| IString::Rc(Arc::<str>::from(s.as_str())))
                    .collect(),
            )
        })
    }

    fn get_property_messages(&self, property_key: &str) -> ErrorMessages {
        self.properties.as_ref().and_then(|m| {
            m.get(property_key).map(|p| {
                IArray::<IString>::Rc(
                    p.errors
                        .iter()
                        .map(|s| IString::Rc(Arc::<str>::from(s.as_str())))
                        .collect(),
                )
            })
        })
    }
}

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

impl From<Errors> for ErrorDetail {
    fn from(value: Errors) -> Self {
        let validation_errors = json!(value);
        let errors = ErrorMessageList::from(&validation_errors["errors"]).unwrap();
        let empty_vec: Vec<Value> = Vec::new();
        let properties: Option<HashMap<String, PropertyError>> =
            validation_errors["properties"].as_object().map(|m| {
                m.iter()
                    .map(|(key, value)| {
                        let new_value = match value {
                            Value::Object(o) => match o.get_key_value("errors") {
                                Some((_, Value::Array(a))) => a,
                                _ => &empty_vec,
                            },
                            _ => &empty_vec,
                        };
                        (key, new_value)
                    })
                    .map(|(k, vec)| {
                        let new_value = vec
                            .iter()
                            .map(|v| match v {
                                Value::String(s) => Some(s),
                                _ => None,
                            })
                            .filter(|i| i.is_some())
                            .map(|o| o.unwrap().to_owned())
                            .collect::<Vec<String>>();
                        (k.to_owned(), PropertyError::from(new_value))
                    })
                    .collect::<HashMap<String, PropertyError>>()
            });
        ErrorDetail { errors, properties }
    }
}
