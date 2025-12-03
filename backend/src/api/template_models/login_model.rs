use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Template)]
#[template(path = "login.html")]
pub struct LoginModel {
    pub token: String,
    pub redirect: String,
}
