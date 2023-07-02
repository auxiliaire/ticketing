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

async fn get_comments(db: Extension<DatabaseConnection>) -> impl IntoResponse {
    Comment::find().all(&*db).await.map_or_else(
        |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
        |list| Json(list).into_response(),
    )
}

async fn get_comment(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    Comment::find_by_id(id).one(&*db).await.map_or_else(
        |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
        |result_option| match result_option {
            Some(comment) => Json(comment).into_response(),
            None => {
                JsonError::from((StatusCode::NOT_FOUND, String::from("Not found"))).into_response()
            }
        },
    )
}

async fn post_comment(
    db: Extension<DatabaseConnection>,
    WithRejection(Json(model), _): WithRejection<Json<comments::Model>, ApiError>,
) -> impl IntoResponse {
    println!("New comment on ticket({})", model.ticket_id);
    comments::ActiveModel {
        text: Set(model.text.to_owned()),
        ticket_id: Set(model.ticket_id.to_owned()),
        user_id: Set(model.user_id.to_owned()),
        ..Default::default()
    }
    .insert(&*db)
    .await
    .map_or_else(
        |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
        |r| Json(r).into_response(),
    )
}

async fn put_comment(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
    WithRejection(Json(update), _): WithRejection<Json<comments::Model>, ApiError>,
) -> impl IntoResponse {
    let original_result = Comment::find_by_id(id).one(&*db).await;
    match original_result {
        Ok(Some(original)) => comments::ActiveModel {
            id: Set(original.id),
            text: Set(update.text.to_owned()),
            ticket_id: Set(original.ticket_id),
            user_id: Set(original.user_id),
            ..Default::default()
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
