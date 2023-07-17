use implicit_clone::sync::{IArray, IString};
use serde_valid::{
    json::{json, Value},
    validation::Errors,
};
use std::sync::Arc;

use super::is_empty::IsEmpty;

pub struct ValidationMessages(pub Option<Vec<String>>);

impl From<&Value> for ValidationMessages {
    fn from(value: &Value) -> Self {
        let empty: Vec<Value> = Vec::new();
        Self(
            value
                .as_array()
                .unwrap_or(&empty)
                .iter()
                .map(|v| match v {
                    Value::String(s) => Some(s.as_str()),
                    _ => None,
                })
                .collect::<Option<Vec<&str>>>()
                .map(|v| v.iter().map(|s| String::from(*s)).collect()),
        )
    }
}

impl ValidationMessages {
    pub fn unwrap(&self) -> Option<Vec<String>> {
        self.0.clone()
    }
}

pub type IValidationMessages = Option<IArray<IString>>;
pub struct ErrorsWrapper(pub Errors);

pub trait ValidationMessagesTrait {
    fn get_common_messages(&self) -> IValidationMessages;
    fn get_property_messages(&self, property_key: &str) -> IValidationMessages;
}

impl ValidationMessagesTrait for ErrorsWrapper {
    fn get_common_messages(&self) -> IValidationMessages {
        let err = json!(self.0);
        ValidationMessages::from(&err["errors"])
            .unwrap()
            .map(|v: Vec<String>| {
                IArray::<IString>::Rc(
                    v.iter()
                        .map(|s| IString::Rc(Arc::<str>::from(s.as_str())))
                        .collect(),
                )
            })
    }

    fn get_property_messages(&self, property_key: &str) -> IValidationMessages {
        let err = json!(self.0);
        ValidationMessages::from(&err["properties"][property_key]["errors"])
            .unwrap()
            .map(|v: Vec<String>| {
                IArray::<IString>::Rc(
                    v.iter()
                        .map(|s| IString::Rc(Arc::<str>::from(s.as_str())))
                        .collect(),
                )
            })
    }
}

impl IsEmpty for IValidationMessages {
    fn is_empty(&self) -> bool {
        self.as_ref().map(|a| a.is_empty()).unwrap_or(true)
    }
}
