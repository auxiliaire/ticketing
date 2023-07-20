use gloo_net::http::Request;
use shared::api::{error::error_response::ErrorResponse, get_api_url};
use shared::dtos::project::Project;
use yew::{platform::spawn_local, Callback};

const PROJECTS_ENDPOINT: &str = "projects";

pub struct ProjectApi;

impl ProjectApi {
    pub fn fetch(id: u64, callback: Callback<Project>) {
        spawn_local(async move {
            let project: Project =
                Request::get(format!("{}{}/{}", get_api_url(), PROJECTS_ENDPOINT, id).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            callback.emit(project);
        });
    }

    pub fn fetch_all(callback: Callback<Vec<Project>>) {
        spawn_local(async move {
            let list: Vec<Project> =
                Request::get(format!("{}{}", get_api_url(), PROJECTS_ENDPOINT).as_str())
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
        project: Project,
        callback: Callback<Project>,
        callback_error: Callback<ErrorResponse>,
    ) {
        spawn_local(async move {
            let res = Request::post(format!("{}{}", get_api_url(), PROJECTS_ENDPOINT).as_str())
                .json(&project)
                .unwrap()
                .send()
                .await;

            match res {
                Ok(resp) => {
                    let text_result = resp.text().await;
                    match text_result {
                        Ok(text) => {
                            let returned_project_result: Result<Project, _> =
                                serde_json::from_str(text.as_str());
                            match returned_project_result {
                                Ok(returned_project) => callback.emit(returned_project),
                                Err(e) => {
                                    log::debug!("Serde result error: {}", e.to_string());
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
