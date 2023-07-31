use sea_orm_migration::prelude::*;

use crate::m20230627_000003_create_ticket_table::Ticket;

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
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
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
                    .drop_column(Alias::new("created_at"))
                    .to_owned(),
            )
            .await
    }
}
