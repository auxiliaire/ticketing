use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(
    Clone, Copy, Debug, Display, EnumIter, EnumString, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum UserRole {
    Developer,
    Manager,
}

pub struct OptionUserRole(pub Option<UserRole>);

impl std::fmt::Display for OptionUserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.map(|r| r.to_string()).unwrap_or(String::from(""))
        )
    }
}

pub struct UserValidation;

impl UserValidation {
    pub fn password_validation(
        password: &Option<String>,
    ) -> Result<(), serde_valid::validation::Error> {
        match password.is_some() && Self::is_password_valid(password.clone().unwrap().as_str()) {
            true => Ok(()),
            false => Err(serde_valid::validation::Error::Custom(
                "Password should contain alphanumeric and special characters at a length range of 8-20.".to_owned(),
            )),
        }
    }

    pub fn is_username_valid(name: &str) -> bool {
        name.len() > 6
    }

    pub fn is_password_valid(password: &str) -> bool {
        let numbers = "0123456789";
        let special_chars = ".:?!{}%#*-_+";
        let lower = "abcdefghijklmnopqrstuvwxyz";
        password.len() > 8
            && password.len() < 20
            && password.contains(|c| numbers.contains(c))
            && password.contains(|c| special_chars.contains(c))
            && password.contains(|c| lower.contains(c))
            && password.contains(|c| lower.to_uppercase().contains(c))
    }

    pub fn are_passwords_matching(
        password1: &str,
        password2: &str,
    ) -> Result<(), serde_valid::validation::Error> {
        match password1 == password2 {
            true => Ok(()),
            false => Err(serde_valid::validation::Error::Custom(
                "Passwords should match.".to_owned(),
            )),
        }
    }

    pub fn role_validation(role: &Option<UserRole>) -> Result<(), serde_valid::validation::Error> {
        match role {
            Some(_) => Ok(()),
            None => Err(serde_valid::validation::Error::Custom(
                String::from("Role should be one of the followings: ")
                    + UserRole::iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                        .as_str()
                    + ".",
            )),
        }
    }
}
