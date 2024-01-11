use axum::{
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use axum_login::login_required;
use http::StatusCode;
use shared::dtos::login_dto::LoginDto;
use crate::api::auth_backend::AuthBackend;
use super::auth_backend::AuthSession;

pub fn router() -> Router {
    Router::new()
        .route("/protected", get(sample_page))
        .route_layer(login_required!(AuthBackend, login_url = "/login"))
        .route("/login", get(login))
        .route("/login", post(do_login))
        .route("/login-success", get(login_success))
}

async fn sample_page() -> impl IntoResponse {
    Html("<h1>Protected Resource!</h1>")
}

async fn login() -> impl IntoResponse {}

async fn do_login(
    mut auth_session: AuthSession,
    Form(login_dto): Form<LoginDto>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(login_dto.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to(
        login_dto
            .redirect
            .unwrap_or(String::from("/login-success"))
            .as_str(),
    )
    .into_response()
}

async fn login_success() -> impl IntoResponse {
    Html("<h2>Login success!</h2>")
}
