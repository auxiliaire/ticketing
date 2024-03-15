pub mod auth_service;
pub mod project_service;
pub mod ticket_service;
pub mod user_service;

const SERVER_URL: &str = env!("SERVER_URL");

pub fn get_api_url() -> String {
    format!("{}/", SERVER_URL)
}
