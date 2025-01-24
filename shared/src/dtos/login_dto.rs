use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
    pub token: String,
    pub redirect: Option<String>,
}
