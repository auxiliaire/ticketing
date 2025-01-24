use super::get_api_url;
use gloo_net::http::Request;
use implicit_clone::unsync::IString;
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::ticket_dto::TicketDto;
use web_sys::{File, FormData};
use yew::{platform::spawn_local, Callback};

const TICKETS_ENDPOINT: &str = "tickets";
const UNASSIGNED_MARKER: &str = "/unassigned";
const SUBSCRIBE_ENDPOINT: &str = "/subscribe";
const IS_SUBSCRIBED_ENDPOINT: &str = "/is_subscribed";
const UPLOAD_ENDPOINT: &str = "/attachments";

pub struct TicketService;

impl TicketService {
    pub fn fetch(jwt: String, id: u64, callback: Callback<TicketDto>) {
        spawn_local(async move {
            let ticket: TicketDto =
                Request::get(format!("{}{}/{}", get_api_url(), TICKETS_ENDPOINT, id).as_str())
                    .header("Authorization", format!("Bearer {}", jwt).as_str())
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
        jwt: String,
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
            let list: Vec<TicketDto> = request_builder
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

    pub fn fetch_unassigned(jwt: String, callback: Callback<Vec<TicketDto>>) {
        spawn_local(async move {
            let list: Vec<TicketDto> = Request::get(
                format!("{}{}{}", get_api_url(), TICKETS_ENDPOINT, UNASSIGNED_MARKER).as_str(),
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

    pub fn create(
        jwt: String,
        ticket: TicketDto,
        callback: Callback<TicketDto>,
        callback_error: Callback<ErrorResponse>,
    ) {
        spawn_local(async move {
            let res = Request::post(format!("{}{}", get_api_url(), TICKETS_ENDPOINT).as_str())
                .header("Authorization", format!("Bearer {}", jwt).as_str())
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
        jwt: String,
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
            .header("Authorization", format!("Bearer {}", jwt).as_str())
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

    pub fn subscribe(jwt: String, id: u64, callback: Callback<bool>) {
        spawn_local(async move {
            if let Ok(response) = Request::post(
                format!(
                    "{}{}/{}{}",
                    get_api_url(),
                    TICKETS_ENDPOINT,
                    id,
                    SUBSCRIBE_ENDPOINT
                )
                .as_str(),
            )
            .header("Authorization", format!("Bearer {}", jwt).as_str())
            .send()
            .await
            {
                callback.emit(response.status().eq(&201_u16));
            }
        });
    }

    pub fn is_subscribed(jwt: String, id: u64, callback: Callback<bool>) {
        spawn_local(async move {
            if let Ok(response) = Request::get(
                format!(
                    "{}{}/{}{}",
                    get_api_url(),
                    TICKETS_ENDPOINT,
                    id,
                    IS_SUBSCRIBED_ENDPOINT
                )
                .as_str(),
            )
            .header("Authorization", format!("Bearer {}", jwt).as_str())
            .send()
            .await
            {
                callback.emit(response.status().eq(&200_u16));
            }
        });
    }

    pub fn upload_attachment(jwt: String, id: u64, file: File, callback: Callback<bool>) {
        spawn_local(async move {
            let payload = FormData::new().unwrap();
            let _ = payload.append_with_blob_and_filename("file", &file, &file.name());
            if let Ok(response) = Request::post(
                format!(
                    "{}{}/{}{}",
                    get_api_url(),
                    TICKETS_ENDPOINT,
                    id,
                    UPLOAD_ENDPOINT
                )
                .as_str(),
            )
            .header("Authorization", format!("Bearer {}", jwt).as_str())
            .body(payload)
            .unwrap()
            .send()
            .await
            {
                callback.emit(response.status().eq(&200_u16));
            }
        });
    }
}
