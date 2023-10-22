use entity::users::{self, Entity as User};
use migration::{ColumnDef, Table};
use sea_orm::{ConnectionTrait, DbConn, DbErr, ExecResult};

pub async fn setup_schema(db: &DbConn) -> Result<ExecResult, DbErr> {
    // Execute create table statement
    db.execute(
        db.get_database_backend().build(
            Table::create()
                .table(User)
                .col(ColumnDef::new(users::Column::Id).integer().primary_key())
                .col(ColumnDef::new(users::Column::Name).string())
                .col(ColumnDef::new(users::Column::Password).string())
                .col(ColumnDef::new(users::Column::Role).string()),
        ),
    )
    .await
}
