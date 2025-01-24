use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Identity {
    pub userid: Uuid,
    pub token: String,
}

impl Display for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.userid.as_hyphenated())
    }
}
