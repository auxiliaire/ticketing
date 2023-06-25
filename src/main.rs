mod api;

use anyhow::Context;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DbErr};

#[tokio::main]
async fn main() {
    let database_url = dotenvy::var("DATABASE_URL")
        .context("DATABASE_URL must be set")
        .unwrap();

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true);

    let pool = Database::connect(opt)
        .await
        .context("Could not make database connection")
        .unwrap();

    Migrator::up(&pool, None)
        .await
        .context("Migration failed")
        .unwrap();

    api::serve(pool).await.unwrap();
}
