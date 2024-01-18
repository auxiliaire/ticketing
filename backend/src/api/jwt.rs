use super::{
    consts::{AUTH_BEARER, JWT_SECRET},
    error::AuthError,
};
use chrono::{Duration, Utc};
use entity::users::Entity as User;
use futures::Future;
use http::{header::AUTHORIZATION, HeaderValue, Request, Response, StatusCode};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tower::{Layer, Service};
use tracing::Instrument;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub uname: String,
}

impl Claims {
    fn try_from_header(auth_header: Option<&HeaderValue>) -> Result<Claims, AuthError> {
        auth_header
            .and_then(|header| std::str::from_utf8(header.as_bytes()).ok())
            .map(|auth_header| auth_header.trim_start_matches(AUTH_BEARER).to_owned())
            .ok_or(AuthError {
                status: StatusCode::BAD_REQUEST,
                message: String::from("No auth token found"),
                code: Some(String::from("Bearer")),
            })
            .and_then(|token| {
                decode_jwt(token).map_err(|_| AuthError {
                    status: StatusCode::UNAUTHORIZED,
                    message: String::from("Unauthorized"),
                    code: Some(String::from("JWT")),
                })
            })
            .map(|jwt| jwt.claims)
    }
}

pub fn encode_jwt(username: String) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claim = Claims {
        iat: now.timestamp() as usize,
        exp: (now + expire).timestamp() as usize,
        uname: username,
    };
    let secret = JWT_SECRET.clone();

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(token: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = JWT_SECRET.clone();
    decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Clone)]
pub struct JwtService<S> {
    pub(crate) inner: S,
}

impl<ReqBody, ResBody, S> Service<Request<ReqBody>> for JwtService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send,
    ReqBody: Send + 'static,
    ResBody: Default + Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let span = tracing::info_span!("call");

        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(
            async move {
                let claims = match Claims::try_from_header(req.headers().get(AUTHORIZATION)) {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::error!(
                            err = %e,
                            "Could not extract claims from header"
                        );
                        let mut res = Response::default();
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return Ok(res);
                    }
                };
                let Some(db) = req.extensions().get::<DatabaseConnection>() else {
                    tracing::error!("Could not get database connection");
                    let mut res = Response::default();
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return Ok(res);
                };
                let Ok(Some(identity)) = User::find()
                    .filter(
                        Condition::all().add(
                            <entity::prelude::Users as EntityTrait>::Column::Name
                                .eq(claims.uname.clone()),
                        ),
                    )
                    .one(db)
                    .await
                else {
                    tracing::error!("Identity could not be established");
                    let mut res = Response::default();
                    *res.status_mut() = http::StatusCode::UNAUTHORIZED;
                    return Ok(res);
                };

                req.extensions_mut().insert(identity);

                let res = inner.call(req).await?;

                Ok(res)
            }
            .instrument(span),
        )
    }
}

#[derive(Clone, Default)]
pub struct JwtLayer {}

impl<S> Layer<S> for JwtLayer {
    type Service = JwtService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        JwtService { inner }
    }
}
