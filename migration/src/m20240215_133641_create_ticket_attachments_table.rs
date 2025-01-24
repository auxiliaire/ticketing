use crate::m20230627_000003_create_ticket_table::Ticket;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TicketAttachment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TicketAttachment::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TicketAttachment::TicketId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-attachments-ticket_id")
                            .from(TicketAttachment::Table, TicketAttachment::TicketId)
                            .to(Ticket::Table, Ticket::Id),
                    )
                    .col(ColumnDef::new(TicketAttachment::LocalPath).string().null())
                    .col(ColumnDef::new(TicketAttachment::Path).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TicketAttachment::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum TicketAttachment {
    #[iden = "ticket_attachments"]
    Table,
    Id,
    TicketId,
    LocalPath,
    Path,
}
