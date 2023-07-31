use crate::m20230627_000003_create_ticket_table::Ticket;
use sea_orm::DeriveActiveEnum;
use sea_orm_migration::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Ticket::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("priority"))
                            .enumeration(Alias::new("Priority"), Priority::iden_values())
                            .default(Priority::Normal),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Ticket::Table)
                    .drop_column(Alias::new("priority"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumIter,
    EnumString,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "Integer", enum_name = "Priority")]
pub enum Priority {
    #[sea_orm(string_value = "Low")]
    Low = 0,
    #[sea_orm(string_value = "Normal")]
    Normal = 10,
    #[sea_orm(string_value = "High")]
    High = 20,
    #[sea_orm(string_value = "Critical")]
    Critical = 30,
}
