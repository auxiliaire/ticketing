use crate::api::error::ApiError;
use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::WithRejection;
use entity::{preferences, preferences::Entity as Preferences, users};
use http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use shared::dtos::preferences_dto::PreferencesDto;

pub fn router() -> Router {
    Router::new()
        .route("/preferences", post(set_preferences))
        .route("/preferences", get(get_preferences))
}

#[derive(Debug, Default, Deserialize, FromQueryResult, Serialize)]
pub struct PreferencesQueryResult {
    pub values: PreferencesDto,
}

#[axum::debug_handler]
async fn get_preferences(
    db: Extension<DatabaseConnection>,
    request_user: Extension<users::Model>,
) -> Result<Json<PreferencesDto>, ApiError> {
    Preferences::find()
        .select_only()
        .column(preferences::Column::Values)
        .filter(preferences::Column::UserId.eq(request_user.id))
        .into_model::<PreferencesQueryResult>()
        .one(&*db)
        .await?
        .map_or(
            Err(ApiError::new(
                StatusCode::NOT_FOUND,
                String::from("Not found"),
            )),
            |preferences| Ok(Json(preferences.values)),
        )
}

async fn set_preferences(
    db: Extension<DatabaseConnection>,
    request_user: Extension<users::Model>,
    WithRejection(Json(update), _): WithRejection<Json<PreferencesDto>, ApiError>,
) -> Result<Json<PreferencesDto>, ApiError> {
    let values = serde_json::to_string(&update)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;
    match Preferences::find()
        .filter(preferences::Column::UserId.eq(request_user.id))
        .one(&*db)
        .await?
    {
        Some(original) => {
            preferences::ActiveModel {
                id: Set(original.id),
                user_id: Set(original.user_id),
                values: Set(values),
            }
            .update(&*db)
            .await?;
        }
        None => {
            preferences::ActiveModel {
                user_id: Set(request_user.id),
                values: Set(values),
                ..Default::default()
            }
            .insert(&*db)
            .await?;
        }
    }
    Ok(Json(update))
}
