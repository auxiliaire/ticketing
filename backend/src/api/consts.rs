use anyhow::Context;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLIENT_URL: String = set_client_url();
    pub static ref STORE_URL: String = set_store_url();
    pub static ref DATABASE_URL: String = set_db_url();
    pub static ref JWT_SECRET: String = set_jwt_secret();
    pub static ref ADMIN_EMAIL: String = set_admin_email();
}

pub const DEFAULT_PAGINATION_OFFSET: u64 = 0;
pub const DEFAULT_PAGINATION_LIMIT: u64 = 5;

fn set_client_url() -> String {
    dotenvy::var("CLIENT_URL")
        .context("CLIENT_URL must be defined in the environment file")
        .unwrap()
}

fn set_store_url() -> String {
    dotenvy::var("REDIS_URL")
        .context("REDIS_URL must be defined in the environment file")
        .unwrap()
}

fn set_db_url() -> String {
    dotenvy::var("DATABASE_URL")
        .context("DATABASE_URL must be defined in the environment file")
        .unwrap()
}

fn set_jwt_secret() -> String {
    dotenvy::var("JWT_SECRET")
        .context("JWT_SECRET must be defined in the environment file")
        .unwrap()
}

fn set_admin_email() -> String {
    dotenvy::var("ADMIN_EMAIL")
        .context("ADMIN_EMAIL must be defined in the environment file")
        .unwrap()
}
