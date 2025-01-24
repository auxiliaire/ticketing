use crate::api::config::MailConfig;
use lettre::{
    transport::smtp::{authentication::Credentials, response::Response, Error},
    Message, SmtpTransport, Transport,
};

#[derive(Clone, Debug)]
pub struct NotificationService {
    config: MailConfig,
}

impl NotificationService {
    pub fn new(config: MailConfig) -> Self {
        Self { config }
    }

    pub fn send_email(&self, email: Message) -> Result<Response, Error> {
        let creds = Credentials::new(self.config.username.clone(), self.config.password.clone());

        let sender = match self.config.tls_off {
            true => SmtpTransport::builder_dangerous(self.config.host.clone())
                .port(self.config.port)
                .credentials(creds)
                .build(),
            false => SmtpTransport::relay(&self.config.host)
                .unwrap()
                .port(self.config.port)
                .credentials(creds)
                .build(),
        };

        sender.send(&email)
    }
}
