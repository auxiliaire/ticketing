use super::{
    auth_backend::AuthSession,
    auth_utils::extract_auth_from_header,
    consts::CLIENT_URL,
    error::{ApiError, AuthError},
    jwt::encode_jwt,
    query,
    services::{
        notification_service::NotificationService, preferences_service::PreferencesService,
    },
    template_models::mfa_email_model::MfaEmailModel,
    validated_json::ValidatedJson,
};
use crate::api::auth_backend::AuthBackend;
use crate::api::template_models::login_model::LoginModel;
use askama_axum::Template;
use axum::{
    body::Body,
    extract::Query,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Extension, Form, Json, Router,
};
use axum_csrf::CsrfToken;
use axum_extra::extract::{
    cookie::{Cookie, CookieJar},
    WithRejection,
};
use axum_login::{login_required, AuthnBackend};
use base64::{engine, Engine};
use entity::{users, users::Model as User};
use http::{HeaderMap, StatusCode};
use lettre::{message::header::ContentType, Message};
use redis::{AsyncCommands, Client};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use shared::{
    api::auth::AuthScheme,
    dtos::{login_dto::LoginDto, user_dto::UserDto},
    validation::user_validation::OptionUserRole,
};
use uuid::Uuid;

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
        .route("/register", post(register))
        .route("/authenticate-cookie", post(authenticate_cookie))
        .route("/authenticate", post(authenticate_raw))
        .route("/refresh-token", get(refresh_token))
}

async fn sample_page() -> impl IntoResponse {
    Html("<h1>Protected Resource!</h1><p>But you have access to it.</p><p><a href=\"/logout\">Log out</a></p>")
}

async fn login(
    token: CsrfToken,
    Query(redirect): Query<query::redirect::Redirect>,
) -> impl IntoResponse {
    let template = LoginModel {
        token: token.authenticity_token().unwrap(),
        redirect: redirect.next.unwrap_or(String::from("/login-success")),
    };

    (token, Body::new(template.render().unwrap())).into_response()
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

async fn register(
    db: Extension<DatabaseConnection>,
    WithRejection(ValidatedJson(model), _): WithRejection<ValidatedJson<UserDto>, ApiError>,
) -> Result<Json<UserDto>, ApiError> {
    println!("User(): '{}'", model.username);
    let user = users::ActiveModel {
        name: Set(model.name.to_owned()),
        username: Set(model.username.to_owned()),
        password: Set(model.password.unwrap().to_owned()),
        role: Set(OptionUserRole(model.role).to_string()),
        ..Default::default()
    }
    .insert(&*db)
    .await?;
    Ok(Json(user.into()))
}

async fn authenticate_cookie(
    headers: HeaderMap,
    Extension(auth_backend): Extension<AuthBackend>,
    jar: CookieJar,
) -> Result<(CookieJar, String), ApiError> {
    let (_, token) = generate_token_using_auth(headers, auth_backend).await?;
    Ok((
        jar.add(
            Cookie::build(("api-token", token))
                .domain("127.0.0.1:8080")
                .path("/")
                .secure(false)
                .http_only(true),
        ),
        String::default(),
    ))
}

async fn authenticate_raw(
    headers: HeaderMap,
    Extension(store): Extension<Client>,
    db: Extension<DatabaseConnection>,
    Extension(auth_backend): Extension<AuthBackend>,
    Extension(notification_service): Extension<NotificationService>,
) -> Result<String, ApiError> {
    // We can just return the token in the body, or create a refresh token instead.
    let (auth_user, token) = generate_token_using_auth(headers, auth_backend).await?;

    // Generate key for token:
    let key = Uuid::new_v4().to_string();

    // Storing the JWT token in cache:
    let mut con = store
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let _: () = con
        .set(key.as_str(), token)
        .await
        .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match PreferencesService::uses_mfa(&db, auth_user.id).await {
        true => send_mfa_message(notification_service, auth_user, key).await,
        false => Ok(key),
    }
}

async fn send_mfa_message(
    notification_service: NotificationService,
    auth_user: User,
    token: String,
) -> Result<String, ApiError> {
    // Prepare email:
    let template = MfaEmailModel {
        client_url: CLIENT_URL.to_string(),
        user_name: auth_user.name,
        token,
    };
    // Send email:
    notification_service
        .send_email(
            Message::builder()
                .from("System <system@example.com>".parse().unwrap())
                .to(auth_user.username.to_string().parse().unwrap())
                .subject("Login Verification")
                .header(ContentType::TEXT_HTML)
                .body(template.render().unwrap())
                .unwrap(),
        )
        .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .map(|_| String::default())
}

async fn refresh_token(
    headers: HeaderMap,
    jar: CookieJar,
    Extension(store): Extension<Client>,
) -> Result<String, ApiError> {
    let bearer = extract_auth_from_header(&headers, AuthScheme::Bearer);
    let refresh_token_cookie = jar.get("refresh-token");

    // Get token from cookie OR bearer:
    let refresh_token = match (refresh_token_cookie, bearer) {
        (Some(refresh_token_cookie), _) => refresh_token_cookie.value().to_owned(),
        (_, Ok(bearer)) => bearer,
        _ => {
            return Err(ApiError::new(
                StatusCode::UNAUTHORIZED,
                String::from("Token is missing"),
            ));
        }
    };

    let mut con = store
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let jwt = con
        .get::<String, String>(refresh_token)
        .await
        .map_err(|e| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error using store: '{}'", e),
            )
        })?;

    Ok(jwt.to_string())
}

async fn generate_token_using_auth(
    headers: HeaderMap,
    auth_backend: AuthBackend,
) -> Result<(User, String), ApiError> {
    let creds_vec =
        extract_auth_from_header(&headers, AuthScheme::Basic).and_then(|base64_string| {
            engine::general_purpose::STANDARD
                .decode(base64_string)
                .map_err(|_| AuthError {
                    status: StatusCode::BAD_REQUEST,
                    message: String::from("Invalid token"),
                    code: Some(AuthScheme::Basic.to_string()),
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

    let token = encode_jwt(user.public_id)
        .map_err(|status| ApiError::new(status, String::from("Token creation error")))?;

    Ok((user, token))
}
