use crate::api::query::helper::empty_string_as_none;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Redirect {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next: Option<String>,
}
