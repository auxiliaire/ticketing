use super::{auth_utils::extract_auth_from_header, consts::JWT_SECRET, error::AuthError};
use chrono::{Duration, Utc};
use entity::users::{self, Entity as User};
use futures::Future;
use http::{HeaderMap, Request, Response, StatusCode};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use shared::api::auth::{AuthScheme, Claims};
use std::pin::Pin;
use tower::{Layer, Service};
use tracing::Instrument;
use uuid::Uuid;

pub fn try_from_header(headers: &HeaderMap) -> Result<Claims, AuthError> {
    extract_auth_from_header(headers, AuthScheme::Bearer)
        .and_then(|token| {
            decode_jwt(token).map_err(|_| AuthError {
                status: StatusCode::UNAUTHORIZED,
                message: String::from("Unauthorized"),
                code: Some(String::from("JWT")),
            })
        })
        .map(|jwt| jwt.claims)
}

pub fn encode_jwt(userid: Uuid) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claim = Claims {
        iat: now.timestamp() as usize,
        exp: (now + expire).timestamp() as usize,
        sub: userid,
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
                let claims = match try_from_header(req.headers()) {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::error!(
                            err = %e,
                            "Could not extract claims from header"
                        );
                        let mut res = Response::default();
                        *res.status_mut() = http::StatusCode::UNAUTHORIZED;
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
                    .filter(users::Column::PublicId.eq(claims.sub))
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
