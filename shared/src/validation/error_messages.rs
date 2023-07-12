use serde_valid::{
    json::{json, Value},
    validation::Errors,
};

pub struct ErrorsWrapper(pub Errors);

pub trait ErrorMessages {
    fn get_common_messages(&self) -> Option<String>;
    fn get_property_messages(&self, property_key: &str) -> Option<String>;
}

impl ErrorMessages for ErrorsWrapper {
    fn get_common_messages(&self) -> Option<String> {
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
            .map(|v| v.join(""))
    }

    fn get_property_messages(&self, property_key: &str) -> Option<String> {
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
            .map(|v| v.join(""))
    }
}
