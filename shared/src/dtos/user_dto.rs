use super::getter::Getter;
use crate::validation::user_validation::{UserRole, UserValidation};
use entity::users::Model;
use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};
use serde_email::Email;
use serde_valid::Validate;
use std::{fmt::Display, rc::Rc};
use strum::{EnumCount, EnumIter, EnumString};
// use serde_with::skip_serializing_none;

#[derive(Copy, Clone, strum::Display, EnumCount, EnumIter, EnumString, PartialEq)]
pub enum UserField {
    Id,
    Name,
    Username,
    Role,
    Action,
}

impl ImplicitClone for UserField {}

impl From<UserField> for usize {
    fn from(val: UserField) -> Self {
        val as usize
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UserValue {
    Id(Rc<Option<u64>>),
    Name(Rc<String>),
    Username(Rc<Email>),
    Role(Rc<Option<UserRole>>),
    Action(Rc<Option<u64>>),
}

impl Display for UserValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserValue::Id(id_ref) => match id_ref.as_ref() {
                Some(id) => write!(f, "{}", id),
                None => write!(f, ""),
            },
            UserValue::Name(name) => write!(f, "{}", name),
            UserValue::Role(role_ref) => match role_ref.as_ref() {
                Some(role) => write!(f, "{}", role),
                None => write!(f, ""),
            },
            UserValue::Action(id_ref) => match id_ref.as_ref() {
                Some(id) => write!(f, "{}", id),
                None => write!(f, ""),
            },
            UserValue::Username(username) => write!(f, "{}", username),
        }
    }
}

pub type IUserDto = Rc<UserDto>;

// Unfortunately #[serde(skip_serializing_if = "Option::is_none")] changes the key in the error
// from field name to "Option::is_none"
// which makes the fields unrecognizable after serialization,
// So this had to be commented out:
// #[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct UserDto {
    pub id: Option<u64>,
    #[validate(min_length = 8)]
    #[validate(max_length = 20)]
    pub name: String,
    pub username: Email,
    #[validate(custom(UserValidation::password_validation))]
    pub password: Option<String>,
    #[validate(custom(UserValidation::role_validation))]
    pub role: Option<UserRole>,
}

impl Getter<UserField, UserValue> for IUserDto {
    fn get(&self, field: UserField) -> UserValue {
        match field {
            UserField::Id => UserValue::Id(Rc::new(self.id)),
            UserField::Name => UserValue::Name(Rc::new(self.name.clone())),
            UserField::Role => UserValue::Role(Rc::new(self.role)),
            UserField::Action => UserValue::Action(Rc::new(self.id)),
            UserField::Username => UserValue::Username(Rc::new(self.username.clone())),
        }
    }
}

impl Display for UserDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( id: {}, name: '{}', username: '{}'  role: {} )",
            self.id.map_or(String::from("-"), |id| format!("{}", id)),
            self.name,
            self.username,
            self.role.map_or(String::from("-"), |r| r.to_string())
        )
    }
}

impl From<&Model> for UserDto {
    fn from(value: &Model) -> Self {
        Self {
            id: Some(value.id),
            name: value.name.to_owned(),
            username: value.username.to_owned(),
            password: None,
            role: UserRole::try_from(value.role.as_str()).ok(),
        }
    }
}

impl From<Model> for UserDto {
    fn from(value: Model) -> Self {
        Self {
            id: Some(value.id),
            name: value.name.to_owned(),
            username: value.username.to_owned(),
            password: None,
            role: UserRole::try_from(value.role.as_str()).ok(),
        }
    }
}

impl UserDto {}

impl ImplicitClone for UserDto {}
