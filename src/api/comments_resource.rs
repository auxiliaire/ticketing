use axum::{
    extract::{rejection::JsonRejection, rejection::PathRejection, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Router,
};
use entity::{comments, comments::Entity as Comment};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::api::error;

#[derive(Serialize, Deserialize)]
struct CommentDto {
    pub text: String,
    pub ticket_id: u64,
    pub user_id: u64,
}

pub fn router() -> Router {
    Router::new()
        .route("/comments", post(post_comment))
        .route("/comments", get(get_comments))
        .route("/comments/:id", get(get_comment))
        .route("/comments/:id", put(put_comment))
        .route("/comments/:id", delete(delete_comment))
}

async fn get_comments(db: Extension<DatabaseConnection>) -> impl IntoResponse {
    let result = Comment::find().all(&*db).await;
    match result {
        Ok(list) => Json(list).into_response(),
        Err(e) => error::to_uniform_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            .into_response(),
    }
}

async fn get_comment(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
) -> impl IntoResponse {
    match param {
        Ok(path) => {
            let result = Comment::find_by_id(path.0).one(&*db).await;
            match result {
                Ok(model) => match model {
                    Some(comment) => Json(comment).into_response(),
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

async fn post_comment(
    db: Extension<DatabaseConnection>,
    payload: Result<Json<CommentDto>, JsonRejection>,
) -> impl IntoResponse {
    match payload {
        Ok(model) => {
            println!("New comment on ticket({})", model.ticket_id);
            let result = comments::ActiveModel {
                text: Set(model.text.to_owned()),
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

async fn put_comment(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
    payload: Result<Json<CommentDto>, JsonRejection>,
) -> impl IntoResponse {
    let original = match param {
        Ok(path) => {
            let result = Comment::find_by_id(path.0).one(&*db).await;
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
            let result = comments::ActiveModel {
                id: Set(o.id),
                text: Set(u.text.to_owned()),
                ticket_id: Set(o.ticket_id),
                user_id: Set(o.user_id),
                ..Default::default()
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

async fn delete_comment(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
) -> impl IntoResponse {
    match param {
        Ok(path) => {
            let ticket_to_be_deleted = comments::ActiveModel {
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
