use crate::api::error::ApiError;
use axum::{
    routing::{get, post, put},
    Extension, Json, Router,
};
use entity::{preferences, preferences::Entity as Preferences, users};
use http::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub fn router() -> Router {
    Router::new()
        //.route("/preferences", post(post_preferences))
        .route("/preferences", get(get_preferences))
    //.route("/preferencess/:id", put(put_preferences))
}

async fn get_preferences(
    db: Extension<DatabaseConnection>,
    request_user: Extension<users::Model>,
) -> Result<Json<preferences::Model>, ApiError> {
    Preferences::find()
        .filter(preferences::Column::UserId.eq(request_user.id))
        .into_model::<preferences::Model>()
        .one(&*db)
        .await?
        .map_or(
            Err(ApiError::new(
                StatusCode::NOT_FOUND,
                String::from("Not found"),
            )),
            |preferences| Ok(Json(preferences)),
        )
}
