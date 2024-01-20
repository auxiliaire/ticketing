use base64::{engine, Engine};
use gloo_net::http::Request;
use shared::api::{error::error_response::ErrorResponse, get_api_url};
use shared::dtos::login_dto::LoginDto;
use yew::{platform::spawn_local, Callback};

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
                            // TODO: Save token to Storage
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
                    "Cookie",
                    &format!("refresh-token={}", refresh_token.as_str()),
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

fn create_token(creds: &LoginDto) -> String {
    let username = creds.username.clone();
    let password = creds.password.clone();
    let con = format!("{}:{}", username, password);
    engine::general_purpose::STANDARD.encode(con)
}
