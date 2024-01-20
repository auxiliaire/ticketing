use base64::{engine, Engine};
use gloo_net::http::Request;
use shared::api::{error::error_response::ErrorResponse, get_api_url};
use shared::dtos::login_dto::LoginDto;
use yew::{platform::spawn_local, Callback};

use crate::app_state::{AppState, AppStateContext};

pub const REFRESH_TOKEN_KEY: &str = "refresh-token";

const AUTHENTICATE_ENDPOINT: &str = "authenticate";
const REFRESH_TOKEN_ENDPOINT: &str = "refresh-token";

pub struct AuthService;

impl AuthService {
    pub fn authenticate(
        creds: LoginDto,
        callback: Callback<LoginDto>,
        callback_error: Callback<ErrorResponse>,
    ) {
        spawn_local(async move {
            // Getting refresh token:
            let res = Request::post(format!("{}{}", get_api_url(), AUTHENTICATE_ENDPOINT).as_str())
                .header(
                    "Authorization",
                    format!("Basic {}", create_token(&creds)).as_str(),
                )
                .send()
                .await;

            let refresh_token = match res {
                Ok(resp) => {
                    let text_result = resp.text().await;
                    match text_result {
                        Ok(token) => {
                            log::debug!("Token: {}", token);
                            let storage =
                                web_sys::window().unwrap().local_storage().unwrap().unwrap();
                            storage.set(REFRESH_TOKEN_KEY, &token).unwrap();
                            Ok(token)
                        }
                        Err(e) => {
                            callback_error.emit(ErrorResponse::from(e.to_string()));
                            Err(e)
                        }
                    }
                }
                Err(e) => {
                    callback_error.emit(ErrorResponse::from(e.to_string()));
                    Err(e)
                }
            }
            .ok()
            .unwrap_or_default();

            // Getting JWT:
            let res = Request::get(format!("{}{}", get_api_url(), REFRESH_TOKEN_ENDPOINT).as_str())
                .header(
                    "Authorization",
                    &format!("Bearer {}", refresh_token.as_str()),
                )
                .send()
                .await;

            match res {
                Ok(resp) => {
                    let text_result = resp.text().await;
                    match text_result {
                        Ok(token) => {
                            log::debug!("JWT: {}", token);
                            callback.emit(LoginDto {
                                username: creds.username,
                                password: String::default(),
                                token,
                                redirect: creds.redirect,
                            });
                        }
                        Err(e) => callback_error.emit(ErrorResponse::from(e.to_string())),
                    }
                }
                Err(e) => callback_error.emit(ErrorResponse::from(e.to_string())),
            };
        });
    }
}

pub async fn try_authenticate_async(app_state: &AppStateContext) -> Result<String, ErrorResponse> {
    if app_state
        .identity
        .as_ref()
        .is_some_and(|i| !i.token.is_empty())
    {
        return Ok(app_state.identity.as_ref().unwrap().token.clone());
    }
    // Looking for refresh token in the local storage:
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let refresh_token = match storage.get(REFRESH_TOKEN_KEY) {
        Ok(Some(t)) => t,
        _ => {
            return Err(ErrorResponse::from(String::from(
                "Could not retrieve refresh token",
            )));
        }
    };

    // Getting JWT:
    let res = Request::get(format!("{}{}", get_api_url(), REFRESH_TOKEN_ENDPOINT).as_str())
        .header(
            "Authorization",
            &format!("Bearer {}", refresh_token.as_str()),
        )
        .send()
        .await;

    match res {
        Ok(resp) => {
            let text_result = resp.text().await;
            match text_result {
                Ok(token) => {
                    log::debug!("JWT: {}", token);
                    AppState::update_identity(
                        app_state,
                        Some(LoginDto {
                            username: String::from("<Faded>"),
                            password: String::default(),
                            token: token.clone(),
                            redirect: None,
                        }),
                    );
                    Ok(token)
                }
                Err(e) => Err(ErrorResponse::from(e.to_string())),
            }
        }
        Err(e) => Err(ErrorResponse::from(e.to_string())),
    }
}

pub fn try_authenticate(app_state: AppStateContext, callback: Callback<Option<String>>) {
    spawn_local(async move {
        callback.emit(try_authenticate_async(&app_state.clone()).await.ok());
    });
}

fn create_token(creds: &LoginDto) -> String {
    let username = creds.username.clone();
    let password = creds.password.clone();
    let con = format!("{}:{}", username, password);
    engine::general_purpose::STANDARD.encode(con)
}
