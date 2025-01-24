use super::get_api_url;
use gloo_net::http::Request;
use implicit_clone::unsync::IString;
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::page::Page;
use shared::dtos::project_dto::{ProjectDto, ProjectTickets};
use shared::dtos::ticket_dto::TicketDto;
use yew::{platform::spawn_local, Callback};

const PROJECTS_ENDPOINT: &str = "projects";
const TICKETS_ENDPOINT: &str = "tickets";

pub struct ProjectService;

impl ProjectService {
    pub fn fetch(jwt: String, id: u64, callback: Callback<ProjectDto>) {
        spawn_local(async move {
            let project: ProjectDto =
                Request::get(format!("{}{}/{}", get_api_url(), PROJECTS_ENDPOINT, id).as_str())
                    .header("Authorization", format!("Bearer {}", jwt).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            callback.emit(project);
        });
    }

    pub fn fetch_all(
        jwt: String,
        sort: Option<IString>,
        order: Option<IString>,
        limit: Option<u64>,
        offset: Option<u64>,
        callback: Callback<Page<ProjectDto>>,
    ) {
        spawn_local(async move {
            let mut request_builder =
                Request::get(format!("{}{}", get_api_url(), PROJECTS_ENDPOINT).as_str());
            if let Some(s) = sort {
                request_builder = request_builder.query([("sort", s.as_str())]);
            }
            if let Some(o) = order {
                request_builder = request_builder.query([("order", o.as_str())]);
            }
            if let Some(l) = limit {
                request_builder = request_builder.query([("limit", format!("{}", l))]);
            }
            if let Some(o) = offset {
                request_builder = request_builder.query([("offset", format!("{}", o))]);
            }
            let page: Page<ProjectDto> = request_builder
                .header("Authorization", format!("Bearer {}", jwt).as_str())
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            callback.emit(page);
        });
    }

    pub fn fetch_latest(jwt: String, callback: Callback<Vec<ProjectDto>>) {
        spawn_local(async move {
            let page: Page<ProjectDto> =
                Request::get(format!("{}{}", get_api_url(), PROJECTS_ENDPOINT).as_str())
                    .query([("limit", "3"), ("sort", "id"), ("order", "desc")])
                    .header("Authorization", format!("Bearer {}", jwt).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            callback.emit(page.list);
        });
    }

    pub fn fetch_assigned_tickets(
        jwt: String,
        project_id: u64,
        callback: Callback<Vec<TicketDto>>,
    ) {
        spawn_local(async move {
            let list: Vec<TicketDto> = Request::get(
                format!(
                    "{}{}/{}/{}",
                    get_api_url(),
                    PROJECTS_ENDPOINT,
                    project_id,
                    TICKETS_ENDPOINT
                )
                .as_str(),
            )
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

    pub fn assign_tickets(
        jwt: String,
        project_id: u64,
        tickets: Vec<u64>,
        callback: Callback<Vec<TicketDto>>,
    ) {
        spawn_local(async move {
            let project_tickets = ProjectTickets { tickets };
            let list: Vec<TicketDto> = Request::post(
                format!(
                    "{}{}/{}/{}",
                    get_api_url(),
                    PROJECTS_ENDPOINT,
                    project_id,
                    TICKETS_ENDPOINT
                )
                .as_str(),
            )
            .header("Authorization", format!("Bearer {}", jwt).as_str())
            .json(&project_tickets)
            .unwrap()
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
        jwt: String,
        project: ProjectDto,
        callback: Callback<ProjectDto>,
        callback_error: Callback<ErrorResponse>,
    ) {
        spawn_local(async move {
            let res = Request::post(format!("{}{}", get_api_url(), PROJECTS_ENDPOINT).as_str())
                .header("Authorization", format!("Bearer {}", jwt).as_str())
                .json(&project)
                .unwrap()
                .send()
                .await;

            match res {
                Ok(resp) => {
                    let text_result = resp.text().await;
                    match text_result {
                        Ok(text) => {
                            let returned_project_result: Result<ProjectDto, _> =
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
