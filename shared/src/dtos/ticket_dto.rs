use crate::validation::ticket_validation::{TicketPriority, TicketStatus};
use entity::{sea_orm_active_enums::Priority, tickets::Model};
use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::{fmt::Display, rc::Rc, str::FromStr};
use strum::{Display, EnumCount, EnumIter, EnumString};

use super::field_index_trait::FieldIndex;

#[derive(Copy, Clone, Display, EnumCount, EnumIter, EnumString, PartialEq)]
pub enum TicketField {
    Id,
    Title,
    Description,
    Project,
    Status,
    User,
    Priority,
}

impl ImplicitClone for TicketField {}

impl FieldIndex for TicketField {
    fn index(&self) -> usize {
        *self as usize
    }
}

pub type ITicketDto = Rc<TicketDto>;

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
