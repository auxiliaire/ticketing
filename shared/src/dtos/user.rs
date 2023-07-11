use std::fmt::Display;

use crate::validation::user::UserValidation;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct User {
    pub id: Option<u64>,
    #[validate(min_length = 8)]
    #[validate(max_length = 20)]
    pub name: String,
    #[validate(
        custom(UserValidation::password_validation),
        message = "Password should contain alphanumeric and special characters at a length range of 8-20."
    )]
    pub password: String,
    #[validate(
        enumerate("Developer", "Manager"),
        message = "User should be either Developer or Manager."
    )]
    pub role: String,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( id: {}, name: {}, role: {})",
            self.id.map_or(String::from(""), |id| format!("{}", id)),
            self.name,
            self.role
        )
    }
}

impl User {
    pub fn new() -> Self {
        User {
            id: None,
            name: String::from(""),
            password: String::from(""),
            role: String::from(""),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User::new()
    }
}
