use std::sync::Arc;

use implicit_clone::sync::{IArray, IString};
use serde_valid::{
    json::{json, Value},
    validation::Errors,
};

pub type ErrorMessages = Option<IArray<IString>>;
pub struct ErrorsWrapper(pub Errors);

pub trait ErrorsTrait {
    fn get_common_messages(&self) -> ErrorMessages;
    fn get_property_messages(&self, property_key: &str) -> ErrorMessages;
}

impl ErrorsTrait for ErrorsWrapper {
    fn get_common_messages(&self) -> ErrorMessages {
        let empty: Vec<Value> = Vec::new();
        let err = json!(self.0);
        err["errors"]
            .as_array()
            .unwrap_or(&empty)
            .iter()
            .map(|v| match v {
                Value::String(s) => Some(s.as_str()),
                _ => None,
            })
            .collect::<Option<Vec<&str>>>()
            .map(|v| v.iter().map(|s| String::from(*s)).collect())
            .map(|v: Vec<String>| {
                IArray::<IString>::Rc(
                    v.iter()
                        .map(|s| IString::Rc(Arc::<str>::from(s.as_str())))
                        .collect(),
                )
            })
    }

    fn get_property_messages(&self, property_key: &str) -> ErrorMessages {
        let empty: Vec<Value> = Vec::new();
        let err = json!(self.0);
        err["properties"][property_key]["errors"]
            .as_array()
            .unwrap_or(&empty)
            .iter()
            .map(|v| match v {
                Value::String(s) => Some(s.as_str()),
                _ => None,
            })
            .collect::<Option<Vec<&str>>>()
            .map(|v| v.iter().map(|s| String::from(*s)).collect())
            .map(|v: Vec<String>| {
                IArray::<IString>::Rc(
                    v.iter()
                        .map(|s| IString::Rc(Arc::<str>::from(s.as_str())))
                        .collect(),
                )
            })
    }
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for ErrorMessages {
    fn is_empty(&self) -> bool {
        self.as_ref().map(|a| a.is_empty()).unwrap_or(true)
    }
}
