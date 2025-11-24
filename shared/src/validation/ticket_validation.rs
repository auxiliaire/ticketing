use entity::sea_orm_active_enums::Priority;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{Display, EnumIter, EnumString};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Display,
    EnumIter,
    EnumString,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum TicketStatus {
    #[default]
    Created,
    Selected,
    Started,
    Reviewing,
    Testing,
    Done,
    Closed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketPriority(pub Priority);

impl Display for TicketPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                Priority::Low => "Low",
                Priority::Normal => "Normal",
                Priority::High => "High",
                Priority::Critical => "Critical",
            }
            .to_owned()
        )
    }
}

impl TryFrom<&str> for TicketPriority {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Low" => Ok(Self(Priority::Low)),
            "Normal" => Ok(Self(Priority::Normal)),
            "High" => Ok(Self(Priority::High)),
            "Critical" => Ok(Self(Priority::Critical)),
            _ => Err(format!("Unknown priority: '{}'", value)),
        }
    }
}

pub struct TicketValidation;

impl TicketValidation {}
