use gloo_net::http::Request;
use shared::api::get_api_url;
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

    pub fn create(user: User, callback: Callback<User>) {
        spawn_local(async move {
            let resp: User = Request::post(format!("{}{}", get_api_url(), USERS_ENDPOINT).as_str())
                .json(&user)
                .unwrap()
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            callback.emit(resp);
        });
    }
}
