use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Router,
};
use axum_extra::extract::WithRejection;
use entity::{projects, projects::Entity as Project};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, Set};

use super::error::{ApiError, JsonError};

pub fn router() -> Router {
    Router::new()
        .route("/projects", post(post_project))
        .route("/projects", get(get_projects))
        .route("/projects/:id", get(get_project))
        .route("/projects/:id", put(put_project))
        .route("/projects/:id", delete(delete_project))
}

async fn get_projects(
    db: Extension<DatabaseConnection>,
) -> Result<Json<Vec<projects::Model>>, ApiError> {
    let list = Project::find().all(&*db).await?;
    Ok(Json(list))
}

async fn get_project(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> Result<Json<projects::Model>, ApiError> {
    Project::find_by_id(id).one(&*db).await?.map_or(
        Err(ApiError::new(
            StatusCode::NOT_FOUND,
            String::from("Not found"),
        )),
        |project| Ok(Json(project)),
    )
}

async fn post_project(
    db: Extension<DatabaseConnection>,
    WithRejection(Json(model), _): WithRejection<Json<projects::Model>, ApiError>,
) -> Result<Json<projects::Model>, ApiError> {
    println!("Project(): '{}'", model.summary);
    let project = projects::ActiveModel {
        summary: Set(model.summary.to_owned()),
        deadline: Set(model.deadline.to_owned()),
        user_id: Set(model.user_id),
        active: Set(model.active),
        ..Default::default()
    }
    .insert(&*db)
    .await?;
    Ok(Json(project))
}

async fn put_project(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
    WithRejection(Json(update), _): WithRejection<Json<projects::Model>, ApiError>,
) -> impl IntoResponse {
    let original_result = Project::find_by_id(id).one(&*db).await?;
    match original_result {
        Some(original) => {
            let updated = projects::ActiveModel {
                id: Set(original.id),
                summary: Set(update.summary.to_owned()),
                deadline: Set(update.deadline.to_owned()),
                user_id: Set(update.user_id),
                active: Set(update.active),
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

async fn delete_project(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    projects::ActiveModel {
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
