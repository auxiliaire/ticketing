//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ticket_updates")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: u64,
    pub ticket_id: u64,
    pub previous_state: String,
    pub next_state: String,
    pub timestamp: DateTime,
    pub user_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tickets::Entity",
        from = "Column::TicketId",
        to = "super::tickets::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Tickets,
}

impl Related<super::tickets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tickets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}