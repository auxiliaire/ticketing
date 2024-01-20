use anyhow::Context;
use api::consts::{DATABASE_URL, STORE_URL};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};

pub mod api;

pub async fn main() {
    let store = redis::Client::open(STORE_URL.clone())
        .context("Could not establish connection to Redis")
        .unwrap();

    let mut opt = ConnectOptions::new(DATABASE_URL.clone());
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

    api::serve(store, pool).await.unwrap();
}
