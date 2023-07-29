use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Clone, Copy, Debug, Display, EnumIter, EnumString, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum TicketStatus {
    Created,
    Selected,
    Started,
    Reviewing,
    Testing,
    Done,
    Closed,
}

impl Default for TicketStatus {
    fn default() -> Self {
        Self::Created
    }
}

pub struct TicketValidation;

impl TicketValidation {}
