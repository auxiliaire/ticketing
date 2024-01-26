use sea_orm_migration::prelude::*;

use crate::m20230627_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-user_public_id")
                    .unique()
                    .table(User::Table)
                    .col(Alias::new("public_id"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx-user_public_id")
                    .table(User::Table)
                    .to_owned(),
            )
            .await
    }
}
