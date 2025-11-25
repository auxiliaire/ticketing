use super::is_empty::IsEmpty;
use implicit_clone::sync::{IArray, IString};
use serde_email::EmailError;
use serde_valid::{
    json::{json, Value},
    validation::Errors,
};
use std::sync::Arc;

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
pub struct ErrorMessage(pub IValidationMessages);

pub trait ValidationMessagesTrait {
    fn get_common_messages(&self) -> IValidationMessages;
    fn get_property_messages(&self, property_key: &str) -> IValidationMessages;
}

impl ValidationMessagesTrait for ErrorsWrapper {
    fn get_common_messages(&self) -> IValidationMessages {
        let err = json!(self.0);
        ValidationMessages::from(&err["errors"])
            .unwrap()
            .map(vec_string_to_immutable)
    }

    fn get_property_messages(&self, property_key: &str) -> IValidationMessages {
        let err = json!(self.0);
        ValidationMessages::from(&err["properties"][property_key]["errors"])
            .unwrap()
            .map(vec_string_to_immutable)
    }
}

impl IsEmpty for IValidationMessages {
    fn is_empty(&self) -> bool {
        self.as_ref().map(|a| a.is_empty()).unwrap_or(true)
    }
}

impl From<ErrorMessage> for IValidationMessages {
    fn from(e: ErrorMessage) -> Self {
        e.0.clone()
    }
}

impl From<EmailError> for ErrorMessage {
    fn from(e: EmailError) -> Self {
        let v = vec![e.to_string()];
        ErrorMessage(Some(vec_string_to_immutable(v)))
    }
}

fn vec_string_to_immutable(v: Vec<String>) -> IArray<IString> {
    IArray::<IString>::Rc(
        v.iter()
            .map(|s| IString::Rc(Arc::<str>::from(s.as_str())))
            .collect(),
    )
}
