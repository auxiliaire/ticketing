use super::getter::Getter;
use crate::validation::project_validation::ProjectValidation;
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use entity::projects::Model;
use implicit_clone::ImplicitClone;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::fmt::Display;
use std::rc::Rc;
use strum::{EnumCount, EnumIter, EnumString};
use uuid::Uuid;

#[derive(Copy, Clone, strum::Display, EnumCount, EnumIter, EnumString, PartialEq)]
pub enum ProjectField {
    Id,
    Summary,
    Deadline,
    Active,
}

impl ImplicitClone for ProjectField {}

impl From<ProjectField> for usize {
    fn from(val: ProjectField) -> Self {
        val as usize
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProjectValue {
    Id(Rc<Option<u64>>),
    Summary(Rc<String>),
    Deadline(Rc<Option<DateTime<Utc>>>),
    Active(i8),
}

impl Display for ProjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectValue::Id(id_ref) => match id_ref.as_ref() {
                Some(id) => write!(f, "{}", id),
                None => write!(f, ""),
            },
            ProjectValue::Summary(summary) => write!(f, "{}", summary),
            ProjectValue::Deadline(deadline_ref) => match deadline_ref.as_ref() {
                Some(deadline) => write!(f, "{}", deadline),
                None => write!(f, ""),
            },
            ProjectValue::Active(active) => write!(f, "{}", active),
        }
    }
}

pub type IProjectDto = Rc<ProjectDto>;

#[derive(Debug, FromQueryResult)]
pub struct ProjectQueryResult {
    pub id: Option<u64>,
    pub summary: String,
    pub deadline: Option<NaiveDate>,
    pub user_id: Uuid,
    pub active: i8,
}

#[derive(
    Clone, Debug, Default, Deserialize, Eq, FromQueryResult, PartialEq, Serialize, Validate,
)]
pub struct ProjectDto {
    pub id: Option<u64>,
    #[validate(min_length = 8)]
    #[validate(max_length = 160)]
    pub summary: String,
    #[serde(with = "ts_seconds_option")]
    #[validate(custom(ProjectValidation::deadline_validation))]
    pub deadline: Option<DateTime<Utc>>,
    pub user_id: Uuid,
    #[validate(enumerate(0, 1), message = "Active can be either 0 or 1.")]
    pub active: i8,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct ProjectTickets {
    pub tickets: Vec<u64>,
}

impl Getter<ProjectField, ProjectValue> for IProjectDto {
    fn get(&self, field: ProjectField) -> ProjectValue {
        match field {
            ProjectField::Id => ProjectValue::Id(Rc::new(self.id)),
            ProjectField::Summary => ProjectValue::Summary(Rc::new(self.summary.clone())),
            ProjectField::Deadline => ProjectValue::Deadline(Rc::new(self.deadline)),
            ProjectField::Active => ProjectValue::Active(self.active),
        }
    }
}

impl Display for ProjectDto {
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

impl From<&Model> for ProjectDto {
    fn from(m: &Model) -> Self {
        Self {
            id: Some(m.id),
            summary: m.summary.to_owned(),
            deadline: m.deadline.map(|d| {
                NaiveDateTime::new(d, NaiveTime::default())
                    .and_local_timezone(Utc)
                    .unwrap()
            }),
            user_id: Uuid::default(),
            active: m.active,
        }
    }
}

impl From<Model> for ProjectDto {
    fn from(m: Model) -> Self {
        Self {
            id: Some(m.id),
            summary: m.summary.to_owned(),
            deadline: m.deadline.map(|d| {
                NaiveDateTime::new(d, NaiveTime::default())
                    .and_local_timezone(Utc)
                    .unwrap()
            }),
            user_id: Uuid::default(),
            active: m.active,
        }
    }
}

impl From<ProjectQueryResult> for ProjectDto {
    fn from(m: ProjectQueryResult) -> Self {
        Self {
            id: m.id,
            summary: m.summary.to_owned(),
            deadline: m.deadline.map(|d| {
                NaiveDateTime::new(d, NaiveTime::default())
                    .and_local_timezone(Utc)
                    .unwrap()
            }),
            user_id: m.user_id,
            active: m.active,
        }
    }
}

impl ProjectDto {}

impl ImplicitClone for ProjectDto {}
