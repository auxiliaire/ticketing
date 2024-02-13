use anyhow::Context;
use api::consts::{DATABASE_URL, POSTGRES_URL, STORE_URL};
use migration::{Migrator, MigratorTrait};
use scheduler::Scheduler;
use sea_orm::{ConnectOptions, Database};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub mod api;
pub mod scheduler;

pub async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Could not set tracing subscriber");

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

    let queue = Scheduler::init(POSTGRES_URL.clone()).await;

    api::serve(store, pool, queue).await.unwrap();
}
