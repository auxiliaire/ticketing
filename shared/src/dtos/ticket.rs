use entity::tickets::Model;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::{fmt::Display, str::FromStr};

use crate::validation::ticket::TicketStatus;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Ticket {
    pub id: Option<u64>,
    #[validate(min_length = 8)]
    #[validate(max_length = 160)]
    pub title: String,
    #[validate(min_length = 8)]
    pub description: String,
    pub project_id: Option<u64>,
    pub status: TicketStatus,
    pub user_id: Option<u64>,
}

impl Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( id: {}, summary: '{}', deadline: {} )",
            self.id.map_or(String::from("-"), |id| format!("{}", id)),
            self.title,
            self.status,
        )
    }
}

impl From<&Model> for Ticket {
    fn from(m: &Model) -> Self {
        Self {
            id: Some(m.id),
            title: m.title.to_owned(),
            description: m.description.to_owned(),
            project_id: m.project_id,
            status: TicketStatus::from_str(m.status.as_str()).unwrap(),
            user_id: m.user_id,
        }
    }
}

impl From<Model> for Ticket {
    fn from(m: Model) -> Self {
        Self {
            id: Some(m.id),
            title: m.title.to_owned(),
            description: m.description.to_owned(),
            project_id: m.project_id,
            status: TicketStatus::from_str(m.status.as_str()).unwrap(),
            user_id: m.user_id,
        }
    }
}

impl Ticket {}