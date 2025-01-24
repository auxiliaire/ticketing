use crate::api::resources::preferences_resource::PreferencesQueryResult;
use entity::{preferences, preferences::Entity as Preferences};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

pub struct PreferencesService {}

impl PreferencesService {
    pub async fn uses_mfa(db: &DatabaseConnection, user_id: u64) -> bool {
        Preferences::find()
            .select_only()
            .column(preferences::Column::Values)
            .filter(preferences::Column::UserId.eq(user_id))
            .into_model::<PreferencesQueryResult>()
            .one(db)
            .await
            .unwrap_or_default()
            .unwrap_or_default()
            .values
            .mfa
            .unwrap_or_default()
    }
}
