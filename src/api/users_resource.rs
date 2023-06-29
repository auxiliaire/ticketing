use axum::{
    extract::{rejection::JsonRejection, rejection::PathRejection, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Router,
};
use entity::{users, users::Entity as User};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::api::error;

pub fn router() -> Router {
    Router::new()
        .route("/users", post(post_user))
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(put_user))
        .route("/users/:id", delete(delete_user))
}

async fn get_users(db: Extension<DatabaseConnection>) -> impl IntoResponse {
    let result = User::find().all(&*db).await;
    match result {
        Ok(list) => Json(list).into_response(),
        Err(e) => error::to_uniform_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            .into_response(),
    }
}

async fn get_user(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
) -> impl IntoResponse {
    match param {
        Ok(path) => {
            let result = User::find_by_id(path.0).one(&*db).await;
            match result {
                Ok(model) => match model {
                    Some(user) => Json(user).into_response(),
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

async fn post_user(
    db: Extension<DatabaseConnection>,
    payload: Result<Json<users::Model>, JsonRejection>,
) -> impl IntoResponse {
    match payload {
        Ok(model) => {
            println!("User(): '{}'", model.name);
            let result = users::ActiveModel {
                name: Set(model.name.to_owned()),
                password: Set(model.password.to_owned()),
                role: Set(model.role.to_owned()),
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

async fn put_user(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
    payload: Result<Json<users::Model>, JsonRejection>,
) -> impl IntoResponse {
    let original = match param {
        Ok(path) => {
            let result = User::find_by_id(path.0).one(&*db).await;
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
            let result = users::ActiveModel {
                id: Set(o.id),
                name: Set(u.name.to_owned()),
                password: Set(u.password.to_owned()),
                role: Set(u.role.to_owned()),
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

async fn delete_user(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
) -> impl IntoResponse {
    match param {
        Ok(path) => {
            let user_to_be_deleted = users::ActiveModel {
                id: sea_orm::ActiveValue::Set(path.0),
                ..Default::default()
            };
            let result = user_to_be_deleted.delete(&*db).await;
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
