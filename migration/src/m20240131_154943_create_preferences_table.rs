use crate::m20230627_000001_create_user_table::User;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Preferences::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Preferences::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Preferences::Values)
                            .json()
                            .not_null()
                            .default("{}"),
                    )
                    .col(
                        ColumnDef::new(Preferences::UserId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-preferences-user_id")
                            .from(Preferences::Table, Preferences::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Preferences::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Preferences {
    Table,
    Id,
    Values,
    UserId,
}
