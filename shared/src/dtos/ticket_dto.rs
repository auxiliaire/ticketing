use crate::validation::ticket_validation::{TicketPriority, TicketStatus};
use entity::{sea_orm_active_enums::Priority, tickets::Model};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::{fmt::Display, str::FromStr};
use strum::{Display, EnumString};

#[derive(Copy, Clone, Display, EnumString)]
pub enum TicketField {
    Title,
    Description,
    Project,
    Status,
    User,
    Priority,
}

impl TicketField {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct TicketDto {
    pub id: Option<u64>,
    #[validate(min_length = 8)]
    #[validate(max_length = 160)]
    pub title: String,
    #[validate(min_length = 8)]
    #[validate(max_length = 500)]
    pub description: String,
    pub project_id: Option<u64>,
    pub status: TicketStatus,
    pub user_id: Option<u64>,
    pub priority: TicketPriority,
}

impl Default for TicketDto {
    fn default() -> Self {
        Self {
            id: Default::default(),
            title: Default::default(),
            description: Default::default(),
            project_id: Default::default(),
            status: Default::default(),
            user_id: Default::default(),
            priority: TicketPriority(Priority::Normal),
        }
    }
}

impl Display for TicketDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( id: {}, title: '{}', status: {} )",
            self.id.map_or(String::from("-"), |id| format!("{}", id)),
            self.title,
            self.status,
        )
    }
}

impl From<&Model> for TicketDto {
    fn from(m: &Model) -> Self {
        Self {
            id: Some(m.id),
            title: m.title.to_owned(),
            description: m.description.to_owned(),
            project_id: m.project_id,
            status: TicketStatus::from_str(m.status.as_str()).unwrap(),
            user_id: m.user_id,
            priority: TicketPriority(m.priority.as_ref().unwrap().to_owned()),
        }
    }
}

impl From<Model> for TicketDto {
    fn from(m: Model) -> Self {
        Self {
            id: Some(m.id),
            title: m.title.to_owned(),
            description: m.description.to_owned(),
            project_id: m.project_id,
            status: TicketStatus::from_str(m.status.as_str()).unwrap(),
            user_id: m.user_id,
            priority: TicketPriority(m.priority.unwrap()),
        }
    }
}

impl TicketDto {}
