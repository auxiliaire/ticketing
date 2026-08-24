use axum_login::{AuthUser, AuthnBackend, UserId};
use entity::users::{self, Entity as User};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use shared::dtos::login_dto::LoginDto;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

pub type AuthSession = axum_login::AuthSession<AuthBackend>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthnUserId {
    pub inner: Uuid,
}

impl Display for AuthnUserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.as_hyphenated())
    }
}

impl From<Uuid> for AuthnUserId {
    fn from(value: Uuid) -> Self {
        Self { inner: value }
    }
}

#[derive(Clone, Debug)]
pub struct AuthnUser {
    pub model: users::Model,
}

impl AuthUser for AuthnUser {
    type Id = AuthnUserId;

    fn id(&self) -> Self::Id {
        self.model.public_id.into()
    }

    fn session_auth_hash(&self) -> &[u8] {
        // TODO: Use a cryptographically sound hashing here:
        self.model.password.as_bytes()
    }
}

impl From<users::Model> for AuthnUser {
    fn from(value: users::Model) -> Self {
        Self { model: value }
    }
}

#[derive(Clone, Debug)]
pub struct AuthBackend {
    db: DatabaseConnection,
}

impl AuthBackend {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl AuthnBackend for AuthBackend {
    type User = AuthnUser;
    type Credentials = LoginDto;
    type Error = DbErr;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = User::find()
            .filter(users::Column::Username.eq(creds.username.clone()))
            .one(&self.db)
            .await?;

        // println!("User: {:?}", user);

        Ok(user
            .filter(|user| {
                // TODO: replace dummy comparison with proper hash based validation:
                creds.password == user.password
                // verify_password(creds.password, &user.password)
                //    .ok()
                //    .is_some()
            })
            .map(|u| u.into()))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = User::find()
            .filter(users::Column::PublicId.eq(user_id.inner))
            .one(&self.db)
            .await?;

        Ok(user.map(|u| u.into()))
    }
}
