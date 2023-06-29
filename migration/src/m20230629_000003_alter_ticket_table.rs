use sea_orm_migration::prelude::*;

use crate::m20230627_000001_create_user_table::User;
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
                    .add_column(ColumnDef::new(Alias::new("user_id")).big_unsigned())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-ticket-user_id")
                            .from_tbl(Ticket::Table)
                            .from_col(Alias::new("user_id"))
                            .to_tbl(User::Table)
                            .to_col(User::Id),
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
                    .drop_foreign_key(Alias::new("fk-ticket-user_id"))
                    .drop_column(Alias::new("user_id"))
                    .to_owned(),
            )
            .await
    }
}
