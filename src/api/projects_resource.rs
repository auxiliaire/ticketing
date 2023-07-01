use axum::{
    extract::{rejection::JsonRejection, rejection::PathRejection, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Router,
};
use entity::{projects, projects::Entity as Project};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::api::error;

pub fn router() -> Router {
    Router::new()
        .route("/projects", post(post_project))
        .route("/projects", get(get_projects))
        .route("/projects/:id", get(get_project))
        .route("/projects/:id", put(put_project))
        .route("/projects/:id", delete(delete_project))
}

async fn get_projects(db: Extension<DatabaseConnection>) -> impl IntoResponse {
    let result = Project::find().all(&*db).await;
    match result {
        Ok(list) => Json(list).into_response(),
        Err(e) => error::to_uniform_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            .into_response(),
    }
}

async fn get_project(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
) -> impl IntoResponse {
    match param {
        Ok(path) => {
            let result = Project::find_by_id(path.0).one(&*db).await;
            match result {
                Ok(model) => match model {
                    Some(project) => Json(project).into_response(),
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

async fn post_project(
    db: Extension<DatabaseConnection>,
    payload: Result<Json<projects::Model>, JsonRejection>,
) -> impl IntoResponse {
    match payload {
        Ok(model) => {
            println!("Project(): '{}'", model.summary);
            let result = projects::ActiveModel {
                summary: Set(model.summary.to_owned()),
                deadline: Set(model.deadline.to_owned()),
                user_id: Set(model.user_id),
                active: Set(model.active),
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

async fn put_project(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
    payload: Result<Json<projects::Model>, JsonRejection>,
) -> impl IntoResponse {
    let original = match param {
        Ok(path) => {
            let result = Project::find_by_id(path.0).one(&*db).await;
            match result {
                Ok(model) => model,
                Err(_) => None,
            }
        }
        Err(_) => None,
    };
    match (original, payload) {
        (Some(o), Ok(u)) => {
            let result = projects::ActiveModel {
                id: Set(o.id),
                summary: Set(u.summary.to_owned()),
                deadline: Set(u.deadline.to_owned()),
                user_id: Set(u.user_id),
                active: Set(u.active),
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

async fn delete_project(
    db: Extension<DatabaseConnection>,
    param: Result<Path<u64>, PathRejection>,
) -> impl IntoResponse {
    match param {
        Ok(path) => {
            let project_to_be_deleted = projects::ActiveModel {
                id: sea_orm::ActiveValue::Set(path.0),
                ..Default::default()
            };
            let result = project_to_be_deleted.delete(&*db).await;
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
