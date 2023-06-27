use sea_orm_migration::prelude::*;

use crate::m20230627_000002_create_project_table::Project;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ticket::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ticket::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Ticket::Title).string().not_null())
                    .col(ColumnDef::new(Ticket::Description).text().not_null())
                    .col(ColumnDef::new(Ticket::ProjectId).big_unsigned())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-ticket-project_id")
                            .from(Ticket::Table, Ticket::ProjectId)
                            .to(Project::Table, Project::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ticket::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Ticket {
    #[iden = "tickets"]
    Table,
    Id,
    Title,
    Description,
    ProjectId,
}
