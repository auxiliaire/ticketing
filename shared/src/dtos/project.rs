use chrono::serde::ts_seconds_option;
use chrono::{DateTime, NaiveDateTime, NaiveTime, Utc};
use entity::projects::Model;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::fmt::Display;

use crate::validation::project::ProjectValidation;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Project {
    pub id: Option<u64>,
    #[validate(min_length = 8)]
    #[validate(max_length = 160)]
    pub summary: String,
    #[serde(with = "ts_seconds_option")]
    #[validate(custom(ProjectValidation::deadline_validation))]
    pub deadline: Option<DateTime<Utc>>,
    pub user_id: u64,
    #[validate(enumerate(0, 1), message = "Active can be either 0 or 1.")]
    pub active: i8,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( id: {}, summary: '{}', deadline: {} )",
            self.id.map_or(String::from("-"), |id| format!("{}", id)),
            self.summary,
            self.deadline.map_or(String::from("-"), |d| d.to_string())
        )
    }
}

impl From<&Model> for Project {
    fn from(m: &Model) -> Self {
        Self {
            id: Some(m.id),
            summary: m.summary.to_owned(),
            deadline: m
                .deadline
                .map(|d| DateTime::from_local(NaiveDateTime::new(d, NaiveTime::default()), Utc)),
            user_id: m.user_id,
            active: m.active,
        }
    }
}

impl From<Model> for Project {
    fn from(m: Model) -> Self {
        Self {
            id: Some(m.id),
            summary: m.summary.to_owned(),
            deadline: m
                .deadline
                .map(|d| DateTime::from_local(NaiveDateTime::new(d, NaiveTime::default()), Utc)),
            user_id: m.user_id,
            active: m.active,
        }
    }
}

impl Project {}
