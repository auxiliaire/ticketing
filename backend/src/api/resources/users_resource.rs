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
use shared::{dtos::user::User as UserDto, validation::user::OptionUserRole};

use crate::api::{
    error::{ApiError, JsonError},
    validated_json::ValidatedJson,
};

pub fn router() -> Router {
    Router::new()
        .route("/users", post(post_user))
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(put_user))
        .route("/users/:id", delete(delete_user))
}

async fn get_users(db: Extension<DatabaseConnection>) -> Result<Json<Vec<users::Model>>, ApiError> {
    let list = User::find().all(&*db).await?;
    Ok(Json(list))
}

async fn get_user(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> Result<Json<users::Model>, ApiError> {
    User::find_by_id(id).one(&*db).await?.map_or(
        Err(ApiError::new(
            StatusCode::NOT_FOUND,
            String::from("Not found"),
        )),
        |user| Ok(Json(user)),
    )
}

async fn post_user(
    db: Extension<DatabaseConnection>,
    WithRejection(ValidatedJson(model), _): WithRejection<ValidatedJson<UserDto>, ApiError>,
) -> Result<Json<users::Model>, ApiError> {
    println!("User(): '{}'", model.name);
    let user = users::ActiveModel {
        name: Set(model.name.to_owned()),
        password: Set(model.password.to_owned()),
        role: Set(OptionUserRole(model.role).to_string()),
        ..Default::default()
    }
    .insert(&*db)
    .await?;
    Ok(Json(user))
}

async fn put_user(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
    WithRejection(Json(update), _): WithRejection<Json<users::Model>, ApiError>,
) -> Result<Json<users::Model>, ApiError> {
    let original_result = User::find_by_id(id).one(&*db).await?;
    match original_result {
        Some(original) => {
            let updated = users::ActiveModel {
                id: Set(original.id),
                name: Set(update.name.to_owned()),
                password: Set(update.password.to_owned()),
                role: Set(update.role.to_owned()),
            }
            .update(&*db)
            .await?;
            Ok(Json(updated))
        }
        None => Err(ApiError::new(
            StatusCode::NOT_FOUND,
            String::from("Not found"),
        )),
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
