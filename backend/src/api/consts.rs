use anyhow::{Context, Error};
use lazy_static::lazy_static;
use std::{
    fs,
    net::{IpAddr, Ipv4Addr},
    num::ParseIntError,
};

lazy_static! {
    pub static ref SERVER_PORT: u16 = set_server_port();
    pub static ref CLIENT_URL: String = set_client_url();
    pub static ref STORE_URL: String = set_store_url();
    pub static ref DATABASE_URL: String = set_db_url();
    pub static ref POSTGRES_URL: String = set_postgres_url();
    pub static ref JWT_SECRET: String = set_jwt_secret();
    pub static ref ADMIN_EMAIL: String = set_admin_email();
    pub static ref MAX_UPLOAD_LIMIT: usize = set_upload_limit();
    pub static ref BUCKET_NAME: String = set_bucket_name();
    pub static ref SMTP_HOST: String = set_smtp_host();
    pub static ref SMTP_PORT: u16 = set_smtp_port();
    pub static ref SMTP_USERNAME: String = set_smtp_username();
    pub static ref SMTP_PASSWORD: String = set_smtp_password();
    pub static ref SMTP_TLS_OFF: bool = set_smtp_tls_off();
}

pub const SERVER_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
pub const DEFAULT_PAGINATION_OFFSET: u64 = 0;
pub const DEFAULT_PAGINATION_LIMIT: u64 = 5;

const DEFAULT_PORT: u16 = 80;

fn set_server_port() -> u16 {
    dotenvy::var("SERVER_PORT")
        .context("SERVER_PORT must be defined in the environment file")
        .and_then(|s| s.parse().map_err(|e: ParseIntError| Error::new(e)))
        .unwrap_or(DEFAULT_PORT)
}

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
    let user = dotenvy::var("MARIADB_USER")
        .context("MARIADB_USER must be defined in the environment file")
        .unwrap();
    let pass = get_from_env_or_file_env("MARIADB_PASSWORD")
        .context("MARIADB_PASSWORD must be defined in the environment file")
        .unwrap();
    let host = dotenvy::var("MARIADB_HOST")
        .context("MARIADB_HOST must be defined in the environment file")
        .unwrap();
    let db = dotenvy::var("MARIADB_DATABASE")
        .context("MARIADB_DATABASE must be defined in the environment file")
        .unwrap();
    format!("mysql://{}:{}@{}/{}", user, pass, host, db)
}

fn set_postgres_url() -> String {
    let user = dotenvy::var("POSTGRES_USER")
        .context("POSTGRES_USER must be defined in the environment file")
        .unwrap();
    let pass = get_from_env_or_file_env("POSTGRES_PASSWORD")
        .context("POSTGRES_PASSWORD must be defined in the environment file")
        .unwrap();
    let host = dotenvy::var("POSTGRES_HOST")
        .context("POSTGRES_HOST must be defined in the environment file")
        .unwrap();
    let port = dotenvy::var("POSTGRES_PORT")
        .context("POSTGRES_PORT must be defined in the environment file")
        .unwrap();
    let db = dotenvy::var("POSTGRES_DB")
        .context("POSTGRES_DB must be defined in the environment file")
        .unwrap();
    format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db)
}

fn set_jwt_secret() -> String {
    get_from_env_or_file_env("JWT_SECRET")
        .context("JWT_SECRET must be defined in the .env file, environment variable, or environment variable pointing to a file")
        .unwrap()
}

fn set_admin_email() -> String {
    dotenvy::var("ADMIN_EMAIL")
        .context("ADMIN_EMAIL must be defined in the environment file")
        .unwrap()
}

fn set_upload_limit() -> usize {
    dotenvy::var("MAX_BODY_LIMIT")
        .context("MAX_BODY_LIMIT must be defined in the environment file")
        .unwrap()
        .parse()
        .context("MAX_BODY_LIMIT must be parsable to a number of type usize")
        .unwrap()
}

fn set_bucket_name() -> String {
    dotenvy::var("BUCKET_NAME")
        .context("BUCKET_NAME must be defined in the environment file")
        .unwrap()
}

fn set_smtp_host() -> String {
    dotenvy::var("SMTP_HOST")
        .context("SMTP_HOST must be defined in the environment file")
        .unwrap()
}

fn set_smtp_port() -> u16 {
    dotenvy::var("SMTP_PORT")
        .context("SMTP_PORT must be defined in the environment file")
        .and_then(|s| s.parse().map_err(|e: ParseIntError| Error::new(e)))
        .unwrap()
}

fn set_smtp_tls_off() -> bool {
    dotenvy::var("SMTP_TLS_OFF").is_ok()
}

fn set_smtp_username() -> String {
    dotenvy::var("SMTP_USERNAME")
        .context("SMTP_USERNAME must be defined in the environment file")
        .unwrap()
}

fn set_smtp_password() -> String {
    get_from_env_or_file_env("SMTP_PASSWORD")
        .context("SMTP_PASSWORD must be defined in the environment file")
        .unwrap()
}

fn get_from_env_or_file_env(key: &str) -> Result<String, Error> {
    dotenvy::var(key).or(get_from_file(
        dotenvy::var(format!("{}_FILE", key)).context("File key not found"),
    ))
}

fn get_from_file(path: Result<String, Error>) -> Result<String, Error> {
    fs::read_to_string(path?).context("File read failed")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &str = "../backend/tests/resources/test.file";
    const TEST_CONTENT: &str = "TEST_CONTENT";
    const TEST_FILE_ERROR: &str = "File read failed";
    const TEST_KEY_ERROR: &str = "File key not found";

    #[test]
    fn test_secret_from_file() -> Result<(), Error> {
        let content = get_from_file(Ok(String::from(TEST_PATH)))?;
        assert_eq!(String::from(TEST_CONTENT), content);
        Ok(())
    }

    #[test]
    fn test_secret_from_file_fail() -> Result<(), Error> {
        let res = get_from_file(Ok(String::from("non-existing-path")));
        assert_eq!(
            String::from(TEST_FILE_ERROR),
            res.expect_err("Unreachable").to_string()
        );
        Ok(())
    }

    #[test]
    fn test_fallback_to_file() -> Result<(), Error> {
        let res = get_from_env_or_file_env("UNSET_TEST_KEY");
        assert_eq!(
            String::from(TEST_KEY_ERROR),
            res.expect_err("Unreachable").to_string()
        );
        Ok(())
    }
}
