use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Router,
};
use axum_extra::extract::WithRejection;
use entity::{comments, comments::Entity as Comment};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use super::error::{ApiError, JsonError};

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

async fn get_comments(
    db: Extension<DatabaseConnection>,
) -> Result<Json<Vec<comments::Model>>, ApiError> {
    let list = Comment::find().all(&*db).await?;
    Ok(Json(list))
}

async fn get_comment(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> Result<Json<comments::Model>, ApiError> {
    Comment::find_by_id(id).one(&*db).await?.map_or(
        Err(ApiError::new(
            StatusCode::NOT_FOUND,
            String::from("Not found"),
        )),
        |comment| Ok(Json(comment)),
    )
}

async fn post_comment(
    db: Extension<DatabaseConnection>,
    WithRejection(Json(model), _): WithRejection<Json<comments::Model>, ApiError>,
) -> Result<Json<comments::Model>, ApiError> {
    println!("New comment on ticket({})", model.ticket_id);
    let comment = comments::ActiveModel {
        text: Set(model.text.to_owned()),
        ticket_id: Set(model.ticket_id.to_owned()),
        user_id: Set(model.user_id.to_owned()),
        ..Default::default()
    }
    .insert(&*db)
    .await?;
    Ok(Json(comment))
}

async fn put_comment(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
    WithRejection(Json(update), _): WithRejection<Json<comments::Model>, ApiError>,
) -> Result<Json<comments::Model>, ApiError> {
    let original_result = Comment::find_by_id(id).one(&*db).await?;
    match original_result {
        Some(original) => {
            let updated = comments::ActiveModel {
                id: Set(original.id),
                text: Set(update.text.to_owned()),
                ticket_id: Set(original.ticket_id),
                user_id: Set(original.user_id),
                ..Default::default()
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

async fn delete_comment(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    comments::ActiveModel {
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
