use std::fmt::Display;

use crate::validation::user::{UserRole, UserValidation};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
#[rule(UserValidation::are_passwords_matching(password, password_repeat))]
pub struct User {
    pub id: Option<u64>,
    #[validate(min_length = 8)]
    #[validate(max_length = 20)]
    pub name: String,
    #[validate(custom(UserValidation::password_validation))]
    pub password: String,
    #[serde(skip_serializing)]
    pub password_repeat: String,
    #[validate(custom(UserValidation::role_validation))]
    pub role: Option<UserRole>,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( id: {}, name: {}, role: {})",
            self.id.map_or(String::from(""), |id| format!("{}", id)),
            self.name,
            self.role
                .map_or(String::from(""), |r| format!("{}", r.to_string()))
        )
    }
}

impl User {
    pub fn new() -> Self {
        User {
            id: None,
            name: String::from(""),
            password: String::from(""),
            password_repeat: String::from(""),
            role: None,
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User::new()
    }
}
