use gloo_net::http::Request;
use implicit_clone::unsync::IString;
use shared::api::{error::error_response::ErrorResponse, get_api_url};
use shared::dtos::ticket_dto::TicketDto;
use yew::{platform::spawn_local, Callback};

const TICKETS_ENDPOINT: &str = "tickets";
const UNASSIGNED_MARKER: &str = "/unassigned";

pub struct TicketService;

impl TicketService {
    pub fn fetch(id: u64, callback: Callback<TicketDto>) {
        spawn_local(async move {
            let ticket: TicketDto =
                Request::get(format!("{}{}/{}", get_api_url(), TICKETS_ENDPOINT, id).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            callback.emit(ticket);
        });
    }

    pub fn fetch_all(
        project_id: Option<u64>,
        search: Option<IString>,
        sort: Option<IString>,
        order: Option<IString>,
        callback: Callback<Vec<TicketDto>>,
    ) {
        spawn_local(async move {
            let mut request_builder =
                Request::get(format!("{}{}", get_api_url(), TICKETS_ENDPOINT).as_str());
            if let Some(p_id) = project_id {
                request_builder = request_builder.query([("project_id", format!("{}", p_id))]);
            }
            if let Some(q) = search {
                request_builder = request_builder.query([("q", q.as_str())]);
            }
            if let Some(s) = sort {
                request_builder = request_builder.query([("sort", s.as_str())]);
            }
            if let Some(o) = order {
                request_builder = request_builder.query([("order", o.as_str())]);
            }
            let list: Vec<TicketDto> = request_builder.send().await.unwrap().json().await.unwrap();

            callback.emit(list);
        });
    }

    pub fn fetch_unassigned(callback: Callback<Vec<TicketDto>>) {
        spawn_local(async move {
            let list: Vec<TicketDto> = Request::get(
                format!("{}{}{}", get_api_url(), TICKETS_ENDPOINT, UNASSIGNED_MARKER).as_str(),
            )
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
        ticket: TicketDto,
        callback: Callback<TicketDto>,
        callback_error: Callback<ErrorResponse>,
    ) {
        spawn_local(async move {
            let res = Request::post(format!("{}{}", get_api_url(), TICKETS_ENDPOINT).as_str())
                .json(&ticket)
                .unwrap()
                .send()
                .await;

            match res {
                Ok(resp) => {
                    let text_result = resp.text().await;
                    match text_result {
                        Ok(text) => {
                            let returned_ticket_result: Result<TicketDto, _> =
                                serde_json::from_str(text.as_str());
                            match returned_ticket_result {
                                Ok(returned_ticket) => callback.emit(returned_ticket),
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

    pub fn update(
        ticket: TicketDto,
        callback: Callback<TicketDto>,
        callback_error: Callback<ErrorResponse>,
    ) {
        spawn_local(async move {
            let res = Request::put(
                format!(
                    "{}{}/{}",
                    get_api_url(),
                    TICKETS_ENDPOINT,
                    ticket.id.unwrap()
                )
                .as_str(),
            )
            .json(&ticket)
            .unwrap()
            .send()
            .await;

            match res {
                Ok(resp) => {
                    let text_result = resp.text().await;
                    match text_result {
                        Ok(text) => {
                            let returned_ticket_result: Result<TicketDto, _> =
                                serde_json::from_str(text.as_str());
                            match returned_ticket_result {
                                Ok(returned_ticket) => callback.emit(returned_ticket),
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
