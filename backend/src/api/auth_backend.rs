use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use entity::users::Entity as User;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use shared::dtos::login_dto::LoginDto;

pub type AuthSession = axum_login::AuthSession<AuthBackend>;

#[derive(Clone, Debug)]
pub struct AuthBackend {
    db: DatabaseConnection,
}

impl AuthBackend {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = entity::users::Model;
    type Credentials = LoginDto;
    type Error = DbErr;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = User::find()
            .filter(Condition::all().add(
                <entity::prelude::Users as EntityTrait>::Column::Name.eq(creds.username.clone()),
            ))
            .one(&self.db)
            .await?;

        // println!("User: {:?}", user);

        Ok(user.filter(|user| {
            // TODO: replace dummy comparison with proper hash based validation:
            creds.password == user.password
            // verify_password(creds.password, &user.password)
            //    .ok()
            //    .is_some()
        }))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = User::find_by_id(*user_id).one(&self.db).await?;

        Ok(user)
    }
}