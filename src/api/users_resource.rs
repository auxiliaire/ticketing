use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Router,
};
use axum_extra::extract::WithRejection;
use entity::{users, users::Entity as User};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, Set};

use super::error::{ApiError, JsonError};

pub fn router() -> Router {
    Router::new()
        .route("/users", post(post_user))
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(put_user))
        .route("/users/:id", delete(delete_user))
}

async fn get_users(db: Extension<DatabaseConnection>) -> impl IntoResponse {
    User::find().all(&*db).await.map_or_else(
        |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
        |list| Json(list).into_response(),
    )
}

async fn get_user(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    User::find_by_id(id).one(&*db).await.map_or_else(
        |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
        |result_option| match result_option {
            Some(user) => Json(user).into_response(),
            None => {
                JsonError::from((StatusCode::NOT_FOUND, String::from("Not found"))).into_response()
            }
        },
    )
}

async fn post_user(
    db: Extension<DatabaseConnection>,
    WithRejection(Json(model), _): WithRejection<Json<users::Model>, ApiError>,
) -> impl IntoResponse {
    println!("User(): '{}'", model.name);
    users::ActiveModel {
        name: Set(model.name.to_owned()),
        password: Set(model.password.to_owned()),
        role: Set(model.role.to_owned()),
        ..Default::default()
    }
    .insert(&*db)
    .await
    .map_or_else(
        |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
        |model| Json(model).into_response(),
    )
}

async fn put_user(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
    WithRejection(Json(update), _): WithRejection<Json<users::Model>, ApiError>,
) -> impl IntoResponse {
    let original_result = User::find_by_id(id).one(&*db).await;
    match original_result {
        Ok(Some(original)) => users::ActiveModel {
            id: Set(original.id),
            name: Set(update.name.to_owned()),
            password: Set(update.password.to_owned()),
            role: Set(update.role.to_owned()),
        }
        .update(&*db)
        .await
        .map_or_else(
            |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
            |r| Json(r).into_response(),
        ),
        Ok(None) => {
            JsonError::from((StatusCode::NOT_FOUND, String::from("Not found"))).into_response()
        }
        Err(e) => {
            JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response()
        }
    }
}

async fn delete_user(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    users::ActiveModel {
        id: Set(id),
        ..Default::default()
    }
    .delete(&*db)
    .await
    .map_or_else(
        |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
        |DeleteResult { rows_affected }| match rows_affected {
            0 => {
                JsonError::from((StatusCode::NOT_FOUND, String::from("Not found"))).into_response()
            }
            n => {
                JsonError::from((StatusCode::NO_CONTENT, format!("Deleted {}", n))).into_response()
            }
        },
    )
}
