use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PropertyError {
    pub errors: Vec<String>,
}

impl From<Vec<String>> for PropertyError {
    fn from(value: Vec<String>) -> Self {
        Self { errors: value }
    }
}
