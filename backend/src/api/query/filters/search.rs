use serde::Deserialize;
use shared::api::helper::empty_string_as_none;

#[derive(Deserialize)]
pub struct Search {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub q: Option<String>,
}
