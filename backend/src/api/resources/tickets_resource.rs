use crate::api::{
    error::{ApiError, JsonError},
    query::{
        filters::{
            pagination::{Pagination, TotalCount},
            search::Search,
            ticket_filter::TicketFilter,
        },
        ordering::Ordering,
    },
    tasks::queue_mailer::{
        push_to_queue, ticket_id_subscriber_set, ticket_update_id, UpdateMessageKey,
    },
};
use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Router,
};
use axum_extra::extract::WithRejection;
use entity::{
    tickets::{self, Entity as Ticket},
    users::{self, Entity as User},
};
use redis::{Client, Commands};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DeleteResult, EntityTrait, Order,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait, RelationTrait, Set,
};
use shared::dtos::{
    page::Page,
    ticket_dto::{TicketDto, TicketQueryResult},
};

pub fn router() -> Router {
    Router::new()
        .route("/tickets", post(post_ticket))
        .route("/tickets", get(get_tickets))
        .route("/tickets/unassigned", get(get_unassigned_tickets))
        .route("/tickets/:id", get(get_ticket))
        .route("/tickets/:id", put(put_ticket))
        .route("/tickets/:id", delete(delete_ticket))
        .route("/tickets/:id/subscribe", post(subscribe_to_ticket))
        .route("/tickets/:id/is_subscribed", get(is_subscribed))
}

async fn get_tickets(
    db: Extension<DatabaseConnection>,
    Query(filter): Query<TicketFilter>,
    Query(search): Query<Search>,
    Query(pagination): Query<Pagination>,
    Query(ordering): Query<Ordering>,
) -> Result<Json<Page<TicketDto>>, ApiError> {
    let total = Ticket::find()
        .select_only()
        .column_as(tickets::Column::Id.count(), "count")
        .into_model::<TotalCount>()
        .one(&*db)
        .await?
        .unwrap()
        .count;
    let mut select = match ordering.sort.and_then(|s| sort_to_column(s.as_str())) {
        Some(sort) => Ticket::find().order_by::<tickets::Column>(sort, ordering.order.0),
        None => Ticket::find().order_by(tickets::Column::Id, Order::Asc),
    };
    select = match search.q {
        Some(q) => select.filter(Condition::all().add(tickets::Column::Title.contains(q))),
        None => select,
    };
    select = match filter.project_id {
        Some(id) => select.filter(
            Condition::all()
                .add(<entity::prelude::Tickets as EntityTrait>::Column::ProjectId.eq(id)),
        ),
        None => select,
    };
    let list = select
        .apply_if(pagination.limit, QuerySelect::limit)
        .offset(pagination.offset)
        .all(&*db)
        .await?;
    Ok(Json(Page::<TicketDto> {
        list: list.iter().map(|m| m.into()).collect::<Vec<TicketDto>>(),
        total,
        offset: pagination.offset.unwrap(),
        limit: pagination.limit.unwrap(),
    }))
}

fn sort_to_column(s: &str) -> Option<tickets::Column> {
    match s {
        "id" => Some(tickets::Column::Id),
        "title" => Some(tickets::Column::Title),
        "description" => Some(tickets::Column::Description),
        "priority" => Some(tickets::Column::Priority),
        "status" => Some(tickets::Column::Status),
        _ => None,
    }
}

async fn get_unassigned_tickets(
    db: Extension<DatabaseConnection>,
) -> Result<Json<Vec<TicketDto>>, ApiError> {
    let list = Ticket::find()
        .filter(
            Condition::all()
                .add(<entity::prelude::Tickets as EntityTrait>::Column::ProjectId.is_null()),
        )
        .all(&*db)
        .await?;
    Ok(Json(
        list.iter().map(|m| m.into()).collect::<Vec<TicketDto>>(),
    ))
}

async fn get_ticket(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> Result<Json<TicketDto>, ApiError> {
    Ticket::find()
        .filter(tickets::Column::Id.eq(id))
        .columns([
            tickets::Column::Id,
            tickets::Column::Title,
            tickets::Column::Description,
            tickets::Column::ProjectId,
            tickets::Column::Status,
            tickets::Column::Priority,
        ])
        .column_as(users::Column::PublicId, "user_id")
        .join(sea_orm::JoinType::LeftJoin, tickets::Relation::Users.def())
        .into_model::<TicketQueryResult>()
        .one(&*db)
        .await?
        .map_or(
            Err(ApiError::new(
                StatusCode::NOT_FOUND,
                String::from("Not found"),
            )),
            |ticket| Ok(Json(ticket.into())),
        )
}

async fn post_ticket(
    db: Extension<DatabaseConnection>,
    WithRejection(Json(model), _): WithRejection<Json<TicketDto>, ApiError>,
) -> Result<Json<TicketDto>, ApiError> {
    println!("Ticket(): '{}'", model.title);

    let user_id = User::find()
        .select_only()
        .column(users::Column::Id)
        .filter(users::Column::PublicId.eq(model.user_id))
        .into_tuple()
        .one(&*db)
        .await?;

    let ticket = tickets::ActiveModel {
        title: Set(model.title.to_owned()),
        description: Set(model.description.to_owned()),
        project_id: Set(model.project_id.to_owned()),
        status: Set(model.status.to_string()),
        user_id: Set(user_id),
        priority: Set(Some(model.priority.0)),
        ..Default::default()
    }
    .insert(&*db)
    .await?;
    Ok(Json(ticket.into()))
}

async fn put_ticket(
    Extension(store): Extension<Client>,
    db: Extension<DatabaseConnection>,
    Extension(auth_user): Extension<users::Model>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
    WithRejection(Json(update), _): WithRejection<Json<TicketDto>, ApiError>,
) -> Result<Json<TicketDto>, ApiError> {
    let original_result = Ticket::find_by_id(id).one(&*db).await?;

    let user_id = User::find()
        .select_only()
        .column(users::Column::Id)
        .filter(users::Column::PublicId.eq(update.user_id))
        .into_tuple()
        .one(&*db)
        .await?;

    match original_result {
        Some(original) => {
            let updated = tickets::ActiveModel {
                id: Set(original.id),
                title: Set(update.title.to_owned()),
                description: Set(update.description.to_owned()),
                status: Set(update.status.to_owned().to_string()),
                project_id: Set(update.project_id),
                user_id: Set(user_id),
                priority: Set(Some(update.priority.0)),
                ..Default::default()
            }
            .update(&*db)
            .await?;

            notify_subscribers(store, auth_user.clone(), updated.clone());

            Ok(Json(updated.into()))
        }
        None => Err(ApiError::new(
            StatusCode::NOT_FOUND,
            String::from("Not found"),
        )),
    }
}

async fn delete_ticket(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    tickets::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
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

async fn subscribe_to_ticket(
    Extension(store): Extension<Client>,
    Extension(auth_user): Extension<users::Model>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    match store.get_connection() {
        Ok(mut con) => {
            let key = ticket_id_subscriber_set(id);
            let member = auth_user.username.to_string();
            match con
                .sismember(&key, &member)
                .and_then(|r: i64| match r {
                    0 => con.sadd(&key, &member).map(|_: i64| StatusCode::CREATED),
                    _ => con.srem(key, member).map(|_: i64| StatusCode::NO_CONTENT),
                })
                .map(|status| JsonError::from((status, String::default())).into_response())
                .map_err(|e| {
                    JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
                        .into_response()
                }) {
                Ok(r) => r,
                Err(e) => e,
            }
        }
        Err(_) => {
            JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, String::default())).into_response()
        }
    }
}

async fn is_subscribed(
    Extension(store): Extension<Client>,
    Extension(auth_user): Extension<users::Model>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    match store.get_connection() {
        Ok(mut con) => {
            con.sismember(ticket_id_subscriber_set(id), auth_user.username.to_string())
                .map_or_else(
                    |e| {
                        JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
                            .into_response()
                    },
                    |r: i64| match r {
                        1 => JsonError::from((StatusCode::OK, String::default())).into_response(),
                        _ => JsonError::from((StatusCode::NOT_FOUND, String::default()))
                            .into_response(),
                    },
                )
        }
        Err(_) => {
            JsonError::from((StatusCode::INTERNAL_SERVER_ERROR, String::default())).into_response()
        }
    }
}

fn notify_subscribers(store: Client, user: users::Model, ticket: tickets::Model) {
    // Notify only if the updater is not the owner
    if user.id.ne(&ticket.user_id.unwrap_or_default()) {
        if let Ok(mut con) = store.get_connection() {
            tokio::spawn(async move {
                // Prepare message
                let subject = format!("Ticket {} has been updated", ticket.id);
                let body = format!(
                    "Ticket update\nTitle: {}\nStatus: {}\nUpdated by: {}",
                    ticket.title.clone(),
                    ticket.status.clone(),
                    user.username.to_string().clone(),
                );
                // Push to queue
                match push_to_queue::<i64>(
                    &mut con,
                    ticket_update_id(ticket.id),
                    &[
                        (
                            UpdateMessageKey::SubscriberSet,
                            ticket_id_subscriber_set(ticket.id),
                        ),
                        (UpdateMessageKey::Subject, subject),
                        (UpdateMessageKey::Body, body),
                    ],
                ) {
                    Ok(_) => (),
                    Err(e) => {
                        tracing::error!("Ticket update message queuing failed: '{}'", e.to_string())
                    }
                }
            });
        }
    }
}
