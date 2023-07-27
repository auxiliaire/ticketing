use crate::validation::user::{UserRole, UserValidation};
use entity::users::Model;
use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use serde_with::skip_serializing_none;
use std::fmt::Display;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct User {
    pub id: Option<u64>,
    #[validate(min_length = 8)]
    #[validate(max_length = 20)]
    pub name: String,
    #[validate(custom(UserValidation::password_validation))]
    pub password: Option<String>,
    #[validate(custom(UserValidation::role_validation))]
    pub role: Option<UserRole>,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( id: {}, name: '{}', role: {} )",
            self.id.map_or(String::from("-"), |id| format!("{}", id)),
            self.name,
            self.role.map_or(String::from("-"), |r| r.to_string())
        )
    }
}

impl From<&Model> for User {
    fn from(value: &Model) -> Self {
        Self {
            id: Some(value.id),
            name: value.name.to_owned(),
            password: None,
            role: UserRole::try_from(value.role.as_str()).ok(),
        }
    }
}

impl From<Model> for User {
    fn from(value: Model) -> Self {
        Self {
            id: Some(value.id),
            name: value.name.to_owned(),
            password: None,
            role: UserRole::try_from(value.role.as_str()).ok(),
        }
    }
}

impl User {}

impl ImplicitClone for User {}
