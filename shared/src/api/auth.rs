use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
pub enum AuthScheme {
    Basic,
    Bearer,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub sub: Uuid,
}
