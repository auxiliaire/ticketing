mod api;

use anyhow::Context;
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() {
    let database_url = dotenvy::var("DATABASE_URL")
        .context("DATABASE_URL must be set")
        .unwrap();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    api::serve(pool).await.unwrap();
}
