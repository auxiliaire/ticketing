use axum::{
    extract::{Json, Path},
    http::StatusCode,
    routing::{get, post},
    Extension, Router,
};
use axum_extra::extract::WithRejection;
use entity::{ticket_updates, ticket_updates::Entity as TicketUpdate};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::api::error::ApiError;

pub fn router() -> Router {
    Router::new()
        .route("/ticket_updates", post(post_ticket_update))
        .route("/ticket_updates", get(get_ticket_updates))
        .route("/ticket_updates/{id}", get(get_ticket_update))
}

async fn get_ticket_updates(
    db: Extension<DatabaseConnection>,
) -> Result<Json<Vec<ticket_updates::Model>>, ApiError> {
    let list = TicketUpdate::find().all(&*db).await?;
    Ok(Json(list))
}

async fn get_ticket_update(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> Result<Json<ticket_updates::Model>, ApiError> {
    TicketUpdate::find_by_id(id).one(&*db).await?.map_or(
        Err(ApiError::new(
            StatusCode::NOT_FOUND,
            String::from("Not found"),
        )),
        |ticket_update| Ok(Json(ticket_update)),
    )
}

async fn post_ticket_update(
    db: Extension<DatabaseConnection>,
    WithRejection(Json(model), _): WithRejection<Json<ticket_updates::Model>, ApiError>,
) -> Result<Json<ticket_updates::Model>, ApiError> {
    println!(
        "TicketUpdate(): {} -> {}",
        model.previous_state, model.next_state
    );
    let ticket_update = ticket_updates::ActiveModel {
        previous_state: Set(model.previous_state.to_owned()),
        next_state: Set(model.next_state.to_owned()),
        ticket_id: Set(model.ticket_id.to_owned()),
        user_id: Set(model.user_id.to_owned()),
        ..Default::default()
    }
    .insert(&*db)
    .await?;
    Ok(Json(ticket_update))
}
