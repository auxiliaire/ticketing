use sea_orm_migration::prelude::*;

use crate::m20230627_000003_create_ticket_table::Ticket;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TicketUpdate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TicketUpdate::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TicketUpdate::TicketId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TicketUpdate::PreviousState)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TicketUpdate::NextState).string().not_null())
                    .col(
                        ColumnDef::new(TicketUpdate::Timestamp)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TicketUpdate::UserId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-ticket_updates-ticket_id")
                            .from(TicketUpdate::Table, TicketUpdate::TicketId)
                            .to(Ticket::Table, Ticket::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TicketUpdate::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum TicketUpdate {
    #[iden = "ticket_updates"]
    Table,
    Id,
    TicketId,
    PreviousState,
    NextState,
    Timestamp,
    UserId,
}
