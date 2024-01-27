use anyhow::Context;

#[derive(Clone, Debug)]
pub struct MailConfig {
    pub host: String,
    pub port: u16,
    pub tls_off: bool,
    pub username: String,
    pub password: String,
}

impl Default for MailConfig {
    fn default() -> Self {
        let host = dotenvy::var("SMTP_HOST")
            .context("SMTP_HOST must be defined in the environment file")
            .unwrap();
        let port = dotenvy::var("SMTP_PORT")
            .context("SMTP_PORT must be defined in the environment file")
            .unwrap();
        let tls_off = dotenvy::var("SMTP_TLS_OFF").is_ok();
        let username = dotenvy::var("SMTP_USERNAME")
            .context("SMTP_USERNAME must be defined in the environment file")
            .unwrap();
        let password = dotenvy::var("SMTP_PASSWORD")
            .context("SMTP_PASSWORD must be defined in the environment file")
            .unwrap();

        Self {
            host,
            port: port
                .parse::<u16>()
                .context("Port must be a valid u16 number")
                .unwrap(),
            tls_off,
            username,
            password,
        }
    }
}
