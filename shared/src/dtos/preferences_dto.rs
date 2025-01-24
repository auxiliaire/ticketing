use crate::api::helper::empty_string_as_default;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Clone, Debug, Default, Deserialize, Display, EnumIter, EnumString, Eq, PartialEq, Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Theme {
    #[default]
    #[strum(serialize = "", serialize = "default")]
    DEFAULT,
    #[strum(to_string = "dark")]
    DARK,
}

impl Theme {
    pub fn flip(previous: Theme) -> Theme {
        match previous {
            Theme::DEFAULT => Theme::DARK,
            Theme::DARK => Theme::DEFAULT,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Notifications {
    pub projects: bool,
    pub tickets: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, FromJsonQueryResult, PartialEq, Serialize)]
pub struct PreferencesDto {
    #[serde(default, deserialize_with = "empty_string_as_default")]
    pub theme: Option<Theme>,
    pub notifications: Option<Notifications>,
    pub mfa: Option<bool>,
}
