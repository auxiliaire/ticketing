use entity::users::{self, Entity as User};
use sea_orm::{ConnectionTrait, DbConn, DbErr, ExecResult};
use migration::{ColumnDef, Table};

pub async fn setup_schema(db: &DbConn) -> Result<ExecResult, DbErr> {
    // Execute create table statement
    db.execute(Table::create()
                .table(User)
                .col(ColumnDef::new(users::Column::Id).integer().primary_key())
                .col(ColumnDef::new(users::Column::Name).string())
                .col(ColumnDef::new(users::Column::Password).string())
                .col(ColumnDef::new(users::Column::Role).string())
                .col(ColumnDef::new(users::Column::PublicId).string())
                .col(ColumnDef::new(users::Column::Username).string())
            )
    .await
}
