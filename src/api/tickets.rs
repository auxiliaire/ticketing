use axum::{
    extract::{rejection::JsonRejection, rejection::PathRejection, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Extension, Router,
};
use entity::{tickets, tickets::Entity as Ticket};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::api::error;

pub fn router() -> Router {
    Router::new()
        .route("/tickets", get(get_tickets))
        .route("/tickets/:id", get(get_ticket))
        .route("/tickets", post(post_ticket))
        .route("/tickets/:id", put(put_ticket))
}

async fn get_tickets(db: Extension<DatabaseConnection>) -> impl IntoResponse {
    let result = Ticket::find().all(&*db).await;
    match result {
        Ok(list) => Json(list).into_response(),
        Err(e) => error::to_uniform_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            .into_response(),
    }
}

async fn get_ticket(
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

async fn post_ticket(
    db: Extension<DatabaseConnection>,
    payload: Result<Json<tickets::Model>, JsonRejection>,
) -> impl IntoResponse {
    match payload {
        Ok(model) => {
            println!("Ticket(): '{}'", model.title);
            let result = tickets::ActiveModel {
                title: Set(model.title.to_owned()),
                description: Set(model.description.to_owned()),
                project_id: Set(model.project_id.to_owned()),
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

async fn put_ticket(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
    payload: Result<Json<tickets::Model>, JsonRejection>,
) -> impl IntoResponse {
    let original = match param {
        Ok(path) => {
            let result = Ticket::find_by_id(path.0).one(&*db).await;
            match result {
                Ok(model) => model,
                Err(_) => None,
            }
        }
        Err(_) => None,
    };
    let update = match payload {
        Ok(model) => Some(model),
        Err(_) => None,
    };
    match (original, update) {
        (Some(o), Some(u)) => {
            let result = tickets::ActiveModel {
                id: Set(o.id),
                title: Set(u.title.to_owned()),
                description: Set(u.description.to_owned()),
                project_id: Set(u.project_id.to_owned()),
            }
            .update(&*db)
            .await;
            match result {
                Ok(r) => Json(r).into_response(),
                Err(e) => error::to_uniform_response(StatusCode::BAD_REQUEST, e.to_string())
                    .into_response(),
            }
        }
        _ => error::to_uniform_response(StatusCode::NOT_FOUND, String::from("Not found"))
            .into_response(),
    }
}
