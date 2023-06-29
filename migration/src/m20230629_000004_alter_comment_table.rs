use sea_orm_migration::prelude::*;

use crate::m20230627_000005_create_comment_table::Comment;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Comment::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("timestamp"))
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Comment::Table)
                    .modify_column(ColumnDef::new(Comment::Timestamp).date_time().not_null())
                    .to_owned(),
            )
            .await
    }
}
