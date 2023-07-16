use gloo_net::http::Request;
use shared::api::{error_response::ErrorResponse, get_api_url};
use shared::dtos::user::User;
use yew::{platform::spawn_local, Callback};

const USERS_ENDPOINT: &str = "users";

pub struct UserApi;

impl UserApi {
    pub fn fetch(id: u64, callback: Callback<User>) {
        spawn_local(async move {
            let user: User =
                Request::get(format!("{}{}/{}", get_api_url(), USERS_ENDPOINT, id).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            callback.emit(user);
        });
    }

    pub fn fetch_all(callback: Callback<Vec<User>>) {
        spawn_local(async move {
            let list: Vec<User> =
                Request::get(format!("{}{}", get_api_url(), USERS_ENDPOINT).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            callback.emit(list);
        });
    }

    pub fn create(user: User, callback: Callback<User>, callback_error: Callback<ErrorResponse>) {
        spawn_local(async move {
            let res = Request::post(format!("{}{}", get_api_url(), USERS_ENDPOINT).as_str())
                .json(&user)
                .unwrap()
                .send()
                .await;

            match res {
                Ok(resp) => {
                    let text_result = resp.text().await;
                    match text_result {
                        Ok(text) => {
                            let returned_user_result: Result<User, _> =
                                serde_json::from_str(text.as_str());
                            match returned_user_result {
                                Ok(returned_user) => callback.emit(returned_user),
                                Err(_) => {
                                    let returned_error_result: Result<ErrorResponse, _> =
                                        serde_json::from_str(text.as_str());
                                    match returned_error_result {
                                        Ok(error_response) => callback_error.emit(error_response),
                                        Err(e) => {
                                            callback_error.emit(ErrorResponse::from(e.to_string()))
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => callback_error.emit(ErrorResponse::from(e.to_string())),
                    }
                }
                Err(e) => callback_error.emit(ErrorResponse::from(e.to_string())),
            }
        });
    }
}
