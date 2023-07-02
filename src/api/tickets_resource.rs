use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Router,
};
use axum_extra::extract::WithRejection;
use entity::{tickets, tickets::Entity as Ticket};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, Set};

use super::error::{ApiError, JsonError};

pub fn router() -> Router {
    Router::new()
        .route("/tickets", post(post_ticket))
        .route("/tickets", get(get_tickets))
        .route("/tickets/:id", get(get_ticket))
        .route("/tickets/:id", put(put_ticket))
        .route("/tickets/:id", delete(delete_ticket))
}

async fn get_tickets(db: Extension<DatabaseConnection>) -> impl IntoResponse {
    Ticket::find().all(&*db).await.map_or_else(
        |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
        |list| Json(list).into_response(),
    )
}

async fn get_ticket(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    Ticket::find_by_id(id).one(&*db).await.map_or_else(
        |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
        |result_option| match result_option {
            Some(ticket) => Json(ticket).into_response(),
            None => {
                JsonError::from((StatusCode::NOT_FOUND, String::from("Not found"))).into_response()
            }
        },
    )
}

async fn post_ticket(
    db: Extension<DatabaseConnection>,
    WithRejection(Json(model), _): WithRejection<Json<tickets::Model>, ApiError>,
) -> impl IntoResponse {
    println!("Ticket(): '{}'", model.title);
    tickets::ActiveModel {
        title: Set(model.title.to_owned()),
        description: Set(model.description.to_owned()),
        project_id: Set(model.project_id.to_owned()),
        ..Default::default()
    }
    .insert(&*db)
    .await
    .map_or_else(
        |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
        |r| Json(r).into_response(),
    )
}

async fn put_ticket(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
    WithRejection(Json(update), _): WithRejection<Json<tickets::Model>, ApiError>,
) -> impl IntoResponse {
    let original_result = Ticket::find_by_id(id).one(&*db).await;
    match original_result {
        Ok(Some(original)) => tickets::ActiveModel {
            id: Set(original.id),
            title: Set(update.title.to_owned()),
            description: Set(update.description.to_owned()),
            status: Set(update.status.to_owned()),
            project_id: Set(update.project_id),
            user_id: Set(update.user_id),
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

async fn delete_ticket(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    tickets::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
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
