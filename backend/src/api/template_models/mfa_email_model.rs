use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Template)]
#[template(path = "mfa_email.html")]
pub struct MfaEmailModel {
    pub client_url: String,
    pub user_name: String,
    pub token: String,
}
