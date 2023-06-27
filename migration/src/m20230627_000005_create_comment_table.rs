use sea_orm_migration::prelude::*;

use crate::{
    m20230627_000001_create_user_table::User, m20230627_000003_create_ticket_table::Ticket,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comment::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comment::Text).text().not_null())
                    .col(ColumnDef::new(Comment::TicketId).big_unsigned().not_null())
                    .col(ColumnDef::new(Comment::Timestamp).date_time().not_null())
                    .col(ColumnDef::new(Comment::UserId).big_unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-ticket_id")
                            .from(Comment::Table, Comment::TicketId)
                            .to(Ticket::Table, Ticket::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-user_id")
                            .from(Comment::Table, Comment::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Comment {
    #[iden = "comments"]
    Table,
    Id,
    Text,
    TicketId,
    Timestamp,
    UserId,
}
