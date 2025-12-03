pub mod auth_service;
pub mod project_service;
pub mod ticket_service;
pub mod user_service;

pub fn get_api_url() -> String {
    String::from("/backend/")
}
