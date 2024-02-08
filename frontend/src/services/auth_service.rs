use crate::app_state::{AppState, AppStateContext};
use crate::helpers::storage_helper::store_in_storage;
use base64::{engine, Engine};
use gloo_net::http::Request;
use shared::api::auth::Claims;
use shared::api::{error::error_response::ErrorResponse, get_api_url};
use shared::dtos::identity::Identity;
use shared::dtos::login_dto::LoginDto;
use uuid::Uuid;
use web_time::{SystemTime, UNIX_EPOCH};
use yew::{platform::spawn_local, Callback};

pub const REFRESH_TOKEN_KEY: &str = "refresh-token";

const AUTHENTICATE_ENDPOINT: &str = "authenticate";
const REFRESH_TOKEN_ENDPOINT: &str = "refresh-token";

pub struct AuthService;

impl AuthService {
    pub fn try_authenticate(app_state: AppStateContext, callback: Callback<Option<String>>) {
        spawn_local(async move {
            callback.emit(try_authenticate_async(&app_state.clone()).await.ok());
        });
    }

    pub fn authenticate(
        creds: LoginDto,
        callback: Callback<Identity>,
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

            let Ok(refresh_token) = (match res {
                Ok(resp) => {
                    let text_result = resp.text().await;
                    match text_result {
                        Ok(token) => {
                            log::debug!("Token: {}", token);
                            match token.is_empty() {
                                true => {
                                    callback_error
                                        .emit(ErrorResponse::from(String::from("Check email")));
                                    Err(String::from("Check email"))
                                }
                                false => {
                                    store_in_storage(
                                        REFRESH_TOKEN_KEY.to_string(),
                                        token.to_string(),
                                    );
                                    Ok(token)
                                }
                            }
                        }
                        Err(e) => {
                            callback_error.emit(ErrorResponse::from(e.to_string()));
                            Err(e.to_string())
                        }
                    }
                }
                Err(e) => {
                    callback_error.emit(ErrorResponse::from(e.to_string()));
                    Err(e.to_string())
                }
            }) else {
                return;
            };

            match fetch_jwt_async(refresh_token).await {
                Ok(identity) => callback.emit(identity),
                Err(e) => callback_error.emit(ErrorResponse::from(e)),
            }
        });
    }

    pub fn fetch_jwt(
        refresh_token: String,
        callback: Callback<Identity>,
        callback_error: Callback<ErrorResponse>,
    ) {
        spawn_local(async move {
            match fetch_jwt_async(refresh_token).await {
                Ok(identity) => callback.emit(identity),
                Err(e) => callback_error.emit(ErrorResponse::from(e)),
            }
        });
    }
}

pub async fn fetch_jwt_async(refresh_token: String) -> Result<Identity, String> {
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
                Ok(token) => match decode_userid(&token) {
                    Ok(userid) => Ok(Identity { userid, token }),
                    Err(e) => {
                        log::error!("Decode error: {}", e);
                        Err(e.to_string())
                    }
                },
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

async fn try_authenticate_async(app_state: &AppStateContext) -> Result<String, ErrorResponse> {
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
                    let userid = decode_userid(&token)?;

                    AppState::update_identity(
                        app_state,
                        Some(Identity {
                            userid,
                            token: token.clone(),
                        }),
                    );
                    Ok(token)
                }
                Err(e) => {
                    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
                    storage.delete(REFRESH_TOKEN_KEY).unwrap_or_default();
                    Err(ErrorResponse::from(e.to_string()))
                }
            }
        }
        Err(e) => Err(ErrorResponse::from(e.to_string())),
    }
}

fn create_token(creds: &LoginDto) -> String {
    let username = creds.username.clone();
    let password = creds.password.clone();
    let con = format!("{}:{}", username, password);
    engine::general_purpose::STANDARD.encode(con)
}

fn decode_userid(token: &str) -> Result<Uuid, String> {
    let split: Vec<&str> = token.split('.').collect();
    let Some(&input) = split.get(1) else {
        return Err(String::from("Invalid structure"));
    };
    let serialized = match engine::general_purpose::URL_SAFE_NO_PAD.decode(input) {
        Ok(vec) => String::from_utf8(vec).map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }?;
    let claims: Claims = serde_json::from_str(&serialized).map_err(|e| e.to_string())?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    match claims.exp > now.try_into().unwrap() {
        true => Ok(claims.sub),
        false => Err(String::from("Token expired")),
    }
}
