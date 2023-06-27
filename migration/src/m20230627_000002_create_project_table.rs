use super::m20230627_000001_create_user_table::User;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Project::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Project::Summary).string().not_null())
                    .col(ColumnDef::new(Project::Deadline).date())
                    .col(ColumnDef::new(Project::UserId).big_unsigned().not_null())
                    .col(ColumnDef::new(Project::Active).boolean().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-project-user_id")
                            .from(Project::Table, Project::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Project {
    #[iden = "projects"]
    Table,
    Id,
    Summary,
    Deadline,
    UserId,
    Active,
}
