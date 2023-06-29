use axum::{
    extract::{rejection::JsonRejection, rejection::PathRejection, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Router,
};
use entity::{tickets, tickets::Entity as Ticket};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::api::error;

pub fn router() -> Router {
    Router::new()
        .route("/tickets", post(post_ticket))
        .route("/tickets", get(get_tickets))
        .route("/tickets/:id", get(get_ticket))
        .route("/tickets/:id", put(put_ticket))
        .route("/tickets/:id", delete(delete_ticket))
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
                status: Set(u.status.to_owned()),
                project_id: Set(u.project_id),
                user_id: Set(u.user_id),
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

async fn delete_ticket(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
) -> impl IntoResponse {
    match param {
        Ok(path) => {
            let ticket_to_be_deleted = tickets::ActiveModel {
                id: sea_orm::ActiveValue::Set(path.0),
                ..Default::default()
            };
            let result = ticket_to_be_deleted.delete(&*db).await;
            match result {
                Ok(delete_result) => match delete_result.rows_affected {
                    0 => {
                        error::to_uniform_response(StatusCode::NOT_FOUND, String::from("Not found"))
                            .into_response()
                    }
                    n => {
                        error::to_uniform_response(StatusCode::NO_CONTENT, format!("Deleted {}", n))
                            .into_response()
                    }
                },
                Err(e) => {
                    error::to_uniform_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                        .into_response()
                }
            }
        }
        Err(e) => {
            error::to_uniform_response(StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}
