pub use sea_orm_migration::prelude::*;

mod m20230627_000001_create_user_table;
mod m20230627_000002_create_project_table;
mod m20230627_000003_create_ticket_table;
mod m20230627_000004_create_ticket_update_table;
mod m20230627_000005_create_comment_table;
mod m20230629_000001_alter_ticket_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230627_000001_create_user_table::Migration),
            Box::new(m20230627_000002_create_project_table::Migration),
            Box::new(m20230627_000003_create_ticket_table::Migration),
            Box::new(m20230627_000004_create_ticket_update_table::Migration),
            Box::new(m20230627_000005_create_comment_table::Migration),
            Box::new(m20230629_000001_alter_ticket_table::Migration),
        ]
    }
}
