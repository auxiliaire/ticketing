//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3, MODIFIED!

use axum_login::AuthUser;
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, DeriveRelation, Related, RelationDef, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};
use serde_email::Email;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Clone, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: u64,
    pub name: String,
    pub password: String,
    pub role: String,
    #[sea_orm(unique)]
    pub username: Email,
    #[sea_orm(unique)]
    pub public_id: Uuid,
}

impl std::fmt::Debug for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.public_id)
            .field("name", &self.name)
            .field("password", &"xxxxxx")
            .field("role", &self.role)
            .finish()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserId {
    pub inner: Uuid,
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.as_hyphenated())
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self { inner: value }
    }
}

impl AuthUser for Model {
    type Id = UserId;

    fn id(&self) -> Self::Id {
        self.public_id.into()
    }

    fn session_auth_hash(&self) -> &[u8] {
        // TODO: Use a cryptographically sound hashing here:
        self.password.as_bytes()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::comments::Entity")]
    Comments,
    #[sea_orm(has_many = "super::preferences::Entity")]
    Preferences,
    #[sea_orm(has_many = "super::projects::Entity")]
    Projects,
    #[sea_orm(has_many = "super::tickets::Entity")]
    Tickets,
}

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<super::preferences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Preferences.def()
    }
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl Related<super::tickets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tickets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            public_id: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}
