//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tickets")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: u64,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub project_id: Option<u64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::comments::Entity")]
    Comments,
    #[sea_orm(
        belongs_to = "super::projects::Entity",
        from = "Column::ProjectId",
        to = "super::projects::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Projects,
    #[sea_orm(has_many = "super::ticket_updates::Entity")]
    TicketUpdates,
}

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl Related<super::ticket_updates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TicketUpdates.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}