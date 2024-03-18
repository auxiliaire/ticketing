use crate::api::{
    error::{ApiError, JsonError},
    query::{
        filters::{pagination::Pagination, search::Search},
        ordering::Ordering,
    },
    validated_json::ValidatedJson,
};
use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Router,
};
use axum_extra::extract::WithRejection;
use entity::{users, users::Entity as User};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DeleteResult, EntityTrait, Order,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait, Set,
};
use shared::{dtos::user_dto::UserDto, validation::user_validation::OptionUserRole};
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/users", post(post_user))
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(put_user))
        .route("/users/:id", delete(delete_user))
}

async fn get_users(
    db: Extension<DatabaseConnection>,
    Query(search): Query<Search>,
    Query(pagination): Query<Pagination>,
    Query(ordering): Query<Ordering>,
) -> Result<Json<Vec<UserDto>>, ApiError> {
    let mut select = match ordering.sort.and_then(|s| sort_to_column(s.as_str())) {
        Some(sort) => User::find().order_by::<users::Column>(sort, ordering.order.0),
        None => User::find().order_by(users::Column::Id, Order::Asc),
    };
    select = match search.q {
        Some(q) => select.filter(Condition::all().add(users::Column::Name.contains(q))),
        None => select,
    };
    let list = select
        .apply_if(pagination.limit, QuerySelect::limit)
        .offset(pagination.offset)
        .all(&*db)
        .await?
        .iter()
        .map(|u| u.into())
        .collect::<Vec<UserDto>>();
    Ok(Json(list))
}

fn sort_to_column(s: &str) -> Option<users::Column> {
    match s {
        "id" => Some(users::Column::Id),
        "name" => Some(users::Column::Name),
        "role" => Some(users::Column::Role),
        _ => None,
    }
}

async fn get_user(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, ApiError>,
) -> Result<Json<UserDto>, ApiError> {
    User::find()
        .filter(users::Column::PublicId.eq(id))
        .one(&*db)
        .await?
        .map_or(
            Err(ApiError::new(
                StatusCode::NOT_FOUND,
                String::from("Not found"),
            )),
            |user| Ok(Json(user.into())),
        )
}

async fn post_user(
    db: Extension<DatabaseConnection>,
    WithRejection(ValidatedJson(model), _): WithRejection<ValidatedJson<UserDto>, ApiError>,
) -> Result<Json<UserDto>, ApiError> {
    println!("User(): '{}'", model.name);
    let user = users::ActiveModel {
        name: Set(model.name.to_owned()),
        username: Set(model.username.to_owned()),
        password: Set(model.password.unwrap().to_owned()),
        role: Set(OptionUserRole(model.role).to_string()),
        ..Default::default()
    }
    .insert(&*db)
    .await?;
    Ok(Json(user.into()))
}

async fn put_user(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
    WithRejection(Json(update), _): WithRejection<Json<UserDto>, ApiError>,
) -> Result<Json<UserDto>, ApiError> {
    let original_result = User::find_by_id(id).one(&*db).await?;
    match original_result {
        Some(original) => {
            let updated = users::ActiveModel {
                id: Set(original.id),
                name: Set(update.name.to_owned()),
                username: Set(original.username),
                password: Set(update.password.unwrap().to_owned()),
                role: Set(update.role.map_or(String::from(""), |r| r.to_string())),
                public_id: Set(update.public_id.unwrap()),
            }
            .update(&*db)
            .await?;
            Ok(Json(updated.into()))
        }
        None => Err(ApiError::new(
            StatusCode::NOT_FOUND,
            String::from("Not found"),
        )),
    }
}

async fn delete_user(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    users::ActiveModel {
        id: Set(id),
        ..Default::default()
    }
    .delete(&*db)
    .await
    .map_or_else(
        |e| JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())).into_response(),
        |DeleteResult { rows_affected }| match rows_affected {
            0 => {
                JsonError::from((StatusCode::NOT_FOUND, String::from("Not found"))).into_response()
            }
            n => {
                JsonError::from((StatusCode::NO_CONTENT, format!("Deleted {}", n))).into_response()
            }
        },
    )
}
