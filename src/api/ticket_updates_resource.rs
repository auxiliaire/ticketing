use axum::{
    extract::{rejection::JsonRejection, rejection::PathRejection, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use entity::{ticket_updates, ticket_updates::Entity as TicketUpdate};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::api::error;

pub fn router() -> Router {
    Router::new()
        .route("/ticket_updates", post(post_ticket_update))
        .route("/ticket_updates", get(get_ticket_updates))
        .route("/ticket_updates/:id", get(get_ticket_update))
}

async fn get_ticket_updates(db: Extension<DatabaseConnection>) -> impl IntoResponse {
    let result = TicketUpdate::find().all(&*db).await;
    match result {
        Ok(list) => Json(list).into_response(),
        Err(e) => error::to_uniform_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            .into_response(),
    }
}

async fn get_ticket_update(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
) -> impl IntoResponse {
    match param {
        Ok(path) => {
            let result = TicketUpdate::find_by_id(path.0).one(&*db).await;
            match result {
                Ok(model) => match model {
                    Some(ticket_update) => Json(ticket_update).into_response(),
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

async fn post_ticket_update(
    db: Extension<DatabaseConnection>,
    payload: Result<Json<ticket_updates::Model>, JsonRejection>,
) -> impl IntoResponse {
    match payload {
        Ok(model) => {
            println!(
                "TicketUpdate(): {} -> {}",
                model.previous_state, model.next_state
            );
            let result = ticket_updates::ActiveModel {
                previous_state: Set(model.previous_state.to_owned()),
                next_state: Set(model.next_state.to_owned()),
                ticket_id: Set(model.ticket_id.to_owned()),
                user_id: Set(model.user_id.to_owned()),
                ..Default::default()
            }
            .insert(&*db)
            .await;
            match result {
                Ok(r) => Json(r).into_response(),
                Err(e) => error::to_uniform_response(StatusCode::BAD_REQUEST, e.to_string())
                    .into_response(),
            }
        }
        Err(e) => error::to_uniform_response(StatusCode::UNPROCESSABLE_ENTITY, e.to_string())
            .into_response(),
    }
}
