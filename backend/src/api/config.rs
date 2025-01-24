use super::consts::{SMTP_HOST, SMTP_PASSWORD, SMTP_PORT, SMTP_TLS_OFF, SMTP_USERNAME};

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
        Self {
            host: SMTP_HOST.to_string(),
            port: *SMTP_PORT,
            tls_off: *SMTP_TLS_OFF,
            username: SMTP_USERNAME.to_string(),
            password: SMTP_PASSWORD.to_string(),
        }
    }
}
