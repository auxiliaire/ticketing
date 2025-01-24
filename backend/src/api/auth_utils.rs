use super::error::AuthError;
use http::{header::AUTHORIZATION, HeaderMap, StatusCode};
use shared::api::auth::AuthScheme;

pub fn extract_auth_from_header(
    headers: &HeaderMap,
    scheme: AuthScheme,
) -> Result<String, AuthError> {
    headers
        .get(AUTHORIZATION)
        .and_then(|header| std::str::from_utf8(header.as_bytes()).ok())
        .map(|auth_header| {
            auth_header
                .trim_start_matches(format!("{} ", scheme).as_str())
                .to_owned()
        })
        .ok_or(AuthError {
            status: StatusCode::BAD_REQUEST,
            message: String::from("No auth token found"),
            code: Some(scheme.to_string()),
        })
}
