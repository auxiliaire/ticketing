use gloo_net::http::Request;
use implicit_clone::unsync::IString;
use shared::api::{error::error_response::ErrorResponse, get_api_url};
use shared::dtos::user_dto::UserDto;
use uuid::Uuid;
use yew::{platform::spawn_local, Callback};

const USERS_ENDPOINT: &str = "users";
const REGISTER_ENDPOINT: &str = "register";

pub struct UserService;

impl UserService {
    pub fn fetch(jwt: String, id: Uuid, callback: Callback<UserDto>) {
        spawn_local(async move {
            let user: UserDto =
                Request::get(format!("{}{}/{}", get_api_url(), USERS_ENDPOINT, id).as_str())
                    .header("Authorization", format!("Bearer {}", jwt).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            callback.emit(user);
        });
    }

    pub fn fetch_all(
        jwt: String,
        search: Option<IString>,
        sort: Option<IString>,
        order: Option<IString>,
        callback: Callback<Vec<UserDto>>,
    ) {
        spawn_local(async move {
            let mut request_builder =
                Request::get(format!("{}{}", get_api_url(), USERS_ENDPOINT).as_str());
            if let Some(q) = search {
                request_builder = request_builder.query([("q", q.as_str())]);
            }
            if let Some(s) = sort {
                request_builder = request_builder.query([("sort", s.as_str())]);
            }
            if let Some(o) = order {
                request_builder = request_builder.query([("order", o.as_str())]);
            }
            let list: Vec<UserDto> = request_builder
                .header("Authorization", format!("Bearer {}", jwt).as_str())
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            callback.emit(list);
        });
    }

    pub fn create(
        user: UserDto,
        callback: Callback<UserDto>,
        callback_error: Callback<ErrorResponse>,
    ) {
        spawn_local(async move {
            let res = Request::post(format!("{}{}", get_api_url(), REGISTER_ENDPOINT).as_str())
                .json(&user)
                .unwrap()
                .send()
                .await;

            match res {
                Ok(resp) => {
                    let text_result = resp.text().await;
                    match text_result {
                        Ok(text) => {
                            let returned_user_result: Result<UserDto, _> =
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
