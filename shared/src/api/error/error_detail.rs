use super::property_error::PropertyError;
use crate::validation::validation_messages::ValidationMessages;
use crate::validation::validation_messages::{IValidationMessages, ValidationMessagesTrait};
use implicit_clone::sync::{IArray, IString};
use serde::{Deserialize, Serialize};
use serde_valid::{
    json::{json, Value},
    validation::Errors,
};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorDetail {
    pub errors: Option<Vec<String>>,
    pub properties: Option<HashMap<String, PropertyError>>,
}

impl ValidationMessagesTrait for ErrorDetail {
    fn get_common_messages(&self) -> IValidationMessages {
        self.errors.as_ref().map(|v| {
            IArray::<IString>::Rc(
                v.iter()
                    .map(|s| IString::Rc(Arc::<str>::from(s.as_str())))
                    .collect(),
            )
        })
    }

    fn get_property_messages(&self, property_key: &str) -> IValidationMessages {
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

impl From<Errors> for ErrorDetail {
    fn from(value: Errors) -> Self {
        let validation_errors = json!(value);
        let errors = ValidationMessages::from(&validation_errors["errors"]).unwrap();
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
