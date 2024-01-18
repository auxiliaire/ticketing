use anyhow::Context;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DATABASE_URL: String = set_db_url();
    pub static ref JWT_SECRET: String = set_jwt_secret();
}

pub const DEFAULT_PAGINATION_OFFSET: u64 = 0;
pub const DEFAULT_PAGINATION_LIMIT: u64 = 5;
pub const AUTH_BASIC: &str = "Basic ";
pub const AUTH_BEARER: &str = "Bearer ";

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
