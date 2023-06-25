use axum::{
    extract::{rejection::JsonRejection, rejection::PathRejection, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use entity::tickets::{self, Entity as Ticket};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::api::error;

#[derive(Serialize, Deserialize)]
pub struct TicketDto {
    id: u64,
    title: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/tickets/:id", get(get_tickets))
        .route("/tickets", post(post_tickets))
}

async fn get_tickets(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
) -> impl IntoResponse {
    match param {
        Ok(path) => {
            let result = Ticket::find_by_id(path.0).one(&*db).await;
            match result {
                Ok(model) => match model {
                    Some(ticket) => Json(TicketDto {
                        id: ticket.id,
                        title: ticket.title,
                    })
                    .into_response(),
                    None => {
                        error::to_uniform_response(StatusCode::NOT_FOUND, String::from("Not found"))
                            .into_response()
                    }
                },
                Err(e) => {
                    error::to_uniform_response(StatusCode::NOT_FOUND, e.to_string()).into_response()
                }
            }
        }
        Err(e) => {
            error::to_uniform_response(StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}

async fn post_tickets(
    db: Extension<DatabaseConnection>,
    payload: Result<Json<TicketDto>, JsonRejection>,
) -> impl IntoResponse {
    match payload {
        Ok(ticket_dto) => {
            println!("Ticket({}): '{}'", ticket_dto.id, ticket_dto.title);
            let ticket = tickets::ActiveModel {
                title: Set(ticket_dto.title.to_owned()),
                ..Default::default()
            };
            let result = tickets::Entity::insert(ticket).exec(&*db).await;
            match result {
                Ok(r) => Json(TicketDto {
                    id: r.last_insert_id,
                    title: String::from(ticket_dto.title.as_str()),
                })
                .into_response(),
                Err(e) => error::to_uniform_response(StatusCode::BAD_REQUEST, e.to_string())
                    .into_response(),
            }
        }
        Err(e) => error::to_uniform_response(StatusCode::UNPROCESSABLE_ENTITY, e.to_string())
            .into_response(),
    }
}
