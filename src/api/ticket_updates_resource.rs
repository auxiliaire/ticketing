use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use axum_extra::extract::WithRejection;
use entity::{ticket_updates, ticket_updates::Entity as TicketUpdate};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use super::error::{ApiError, JsonError};

pub fn router() -> Router {
    Router::new()
        .route("/ticket_updates", post(post_ticket_update))
        .route("/ticket_updates", get(get_ticket_updates))
        .route("/ticket_updates/:id", get(get_ticket_update))
}

async fn get_ticket_updates(db: Extension<DatabaseConnection>) -> impl IntoResponse {
    TicketUpdate::find().all(&*db).await.map_or_else(
        |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
        |list| Json(list).into_response(),
    )
}

async fn get_ticket_update(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    TicketUpdate::find_by_id(id).one(&*db).await.map_or_else(
        |e| JsonError::from((StatusCode::NOT_FOUND, e.to_string())).into_response(),
        |model| match model {
            Some(ticket_update) => Json(ticket_update).into_response(),
            None => {
                JsonError::from((StatusCode::NOT_FOUND, String::from("Not found"))).into_response()
            }
        },
    )
}

async fn post_ticket_update(
    db: Extension<DatabaseConnection>,
    WithRejection(Json(model), _): WithRejection<Json<ticket_updates::Model>, ApiError>,
) -> impl IntoResponse {
    println!(
        "TicketUpdate(): {} -> {}",
        model.previous_state, model.next_state
    );
    ticket_updates::ActiveModel {
        previous_state: Set(model.previous_state.to_owned()),
        next_state: Set(model.next_state.to_owned()),
        ticket_id: Set(model.ticket_id.to_owned()),
        user_id: Set(model.user_id.to_owned()),
        ..Default::default()
    }
    .insert(&*db)
    .await
    .map_or_else(
        |e| JsonError::from((StatusCode::BAD_REQUEST, e.to_string())).into_response(),
        |r| Json(r).into_response(),
    )
}
