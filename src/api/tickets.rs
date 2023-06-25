use axum::{
    extract::{rejection::JsonRejection, rejection::PathRejection, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use entity::{ticket, ticket::Entity as Ticket};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::api::error;

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
                    Some(ticket) => Json(ticket).into_response(),
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
    payload: Result<Json<ticket::Model>, JsonRejection>,
) -> impl IntoResponse {
    match payload {
        Ok(model) => {
            println!("Ticket(): '{}'", model.title);
            let result = ticket::ActiveModel {
                title: Set(model.title.to_owned()),
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
