use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
    pub token: String,
    pub redirect: Option<String>,
}
