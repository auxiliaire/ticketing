use super::{
    auth_backend::AuthSession,
    consts::AUTH_BASIC,
    error::{ApiError, AuthError},
    jwt::encode_jwt,
    query,
};
use crate::api::auth_backend::AuthBackend;
use askama::Template;
use axum::{
    extract::Query,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Extension, Form, Router,
};
use axum_csrf::CsrfToken;
use axum_login::{login_required, AuthnBackend};
use base64::{engine, Engine};
use http::{header::AUTHORIZATION, HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use shared::dtos::login_dto::LoginDto;

pub fn router() -> Router {
    Router::new()
        // Login protected demo page
        .route("/protected", get(sample_page))
        .route_layer(login_required!(AuthBackend, login_url = "/login"))
        // Web routes
        .route("/login", get(login))
        .route("/login", post(do_login))
        .route("/login-success", get(login_success))
        .route("/logout", get(logout))
        // API routes
        .route("/authenticate", post(authenticate))
}

#[derive(Deserialize, Serialize, Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    token: String,
    redirect: String,
}

async fn sample_page() -> impl IntoResponse {
    Html("<h1>Protected Resource!</h1><p>But you have access to it.</p><p><a href=\"/logout\">Log out</a></p>")
}

async fn login(
    token: CsrfToken,
    Query(redirect): Query<query::redirect::Redirect>,
) -> impl IntoResponse {
    let template = LoginTemplate {
        token: token.authenticity_token().unwrap(),
        redirect: redirect.next.unwrap_or(String::from("/login-success")),
    };

    (token, template).into_response()
}

async fn do_login(
    mut auth_session: AuthSession,
    token: CsrfToken,
    Form(login_dto): Form<LoginDto>,
) -> impl IntoResponse {
    if token.verify(&login_dto.token).is_err() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

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

async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn authenticate(
    headers: HeaderMap,
    Extension(auth_backend): Extension<AuthBackend>,
) -> Result<String, ApiError> {
    let creds_vec = headers
        .get(AUTHORIZATION)
        .and_then(|header| std::str::from_utf8(header.as_bytes()).ok())
        .map(|auth_header| auth_header.trim_start_matches(AUTH_BASIC).to_owned())
        .ok_or(AuthError {
            status: StatusCode::BAD_REQUEST,
            message: String::from("No auth token found"),
            code: Some(String::from("Bearer")),
        })
        .and_then(|base64_string| {
            engine::general_purpose::STANDARD
                .decode(base64_string)
                .map_err(|_| AuthError {
                    status: StatusCode::BAD_REQUEST,
                    message: String::from("Invalid token"),
                    code: Some(String::from("Basic")),
                })
        })?;
    let creds: Vec<&str> = std::str::from_utf8(&creds_vec)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?
        .split(':')
        .collect();
    let login_dto = LoginDto {
        username: String::from(creds[0]),
        password: String::from(creds[1]),
        token: String::default(),
        redirect: None,
    };
    let Some(user) = auth_backend.authenticate(login_dto).await? else {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            String::from("Invalid credentials"),
        ));
    };

    let token = encode_jwt(user.name)
        .map_err(|status| ApiError::new(status, String::from("Token creation error")))?;
    Ok(token)
}