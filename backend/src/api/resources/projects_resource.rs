use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Router,
};
use axum_extra::extract::WithRejection;
use entity::tickets::Entity as Ticket;
use entity::{projects, projects::Entity as Project};
use migration::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DeleteResult, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait, Set,
};
use shared::dtos::project_dto::ProjectTickets as ProjectTicketsDto;
use shared::dtos::ticket_dto::TicketDto;
use shared::{dtos::project_dto::ProjectDto, validation::ticket_validation::TicketStatus};

use crate::api::{
    error::{ApiError, JsonError},
    query::{filters::pagination::Pagination, ordering::Ordering},
    validated_json::ValidatedJson,
};

pub fn router() -> Router {
    Router::new()
        .route("/projects", post(post_project))
        .route("/projects", get(get_projects))
        .route("/projects/:id", get(get_project))
        .route("/projects/:id/tickets", get(get_project_tickets))
        .route("/projects/:id/tickets", post(post_project_tickets))
        .route("/projects/:id", put(put_project))
        .route("/projects/:id", delete(delete_project))
}

async fn get_projects(
    db: Extension<DatabaseConnection>,
    Query(pagination): Query<Pagination>,
    Query(ordering): Query<Ordering>,
) -> Result<Json<Vec<ProjectDto>>, ApiError> {
    let mut select = Project::find();
    if let Some(sort) = ordering.sort.and_then(|s| sort_to_column(s.as_str())) {
        select = select.order_by::<projects::Column>(sort, ordering.order.0);
    }
    let list = select
        .apply_if(pagination.limit, QuerySelect::limit)
        .offset(pagination.offset)
        .all(&*db)
        .await?
        .iter()
        .map(|m| m.into())
        .collect::<Vec<ProjectDto>>();
    Ok(Json(list))
}

fn sort_to_column(s: &str) -> Option<projects::Column> {
    match s {
        "id" => Some(projects::Column::Id),
        "summary" => Some(projects::Column::Summary),
        "deadline" => Some(projects::Column::Deadline),
        _ => None,
    }
}

async fn get_project(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> Result<Json<ProjectDto>, ApiError> {
    Project::find_by_id(id).one(&*db).await?.map_or(
        Err(ApiError::new(
            StatusCode::NOT_FOUND,
            String::from("Not found"),
        )),
        |project| Ok(Json(project.into())),
    )
}

async fn get_project_tickets(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> Result<Json<Vec<TicketDto>>, ApiError> {
    let list = Ticket::find()
        .filter(
            Condition::all()
                .add(<entity::prelude::Tickets as EntityTrait>::Column::ProjectId.eq(id)),
        )
        .all(&*db)
        .await?;
    Ok(Json(
        list.iter().map(|m| m.into()).collect::<Vec<TicketDto>>(),
    ))
}

async fn post_project_tickets(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
    WithRejection(ValidatedJson(tickets_dto), _): WithRejection<
        ValidatedJson<ProjectTicketsDto>,
        ApiError,
    >,
) -> Result<Json<Vec<TicketDto>>, ApiError> {
    Ticket::update_many()
        .col_expr(
            <entity::prelude::Tickets as EntityTrait>::Column::ProjectId,
            Expr::value(id),
        )
        .col_expr(
            <entity::prelude::Tickets as EntityTrait>::Column::Status,
            Expr::value(TicketStatus::Selected.to_string()),
        )
        .filter(<entity::prelude::Tickets as EntityTrait>::Column::Id.is_in(tickets_dto.tickets))
        .exec(&*db)
        .await?;
    let list = Ticket::find()
        .filter(
            Condition::all()
                .add(<entity::prelude::Tickets as EntityTrait>::Column::ProjectId.eq(id)),
        )
        .all(&*db)
        .await?;
    Ok(Json(
        list.iter().map(|m| m.into()).collect::<Vec<TicketDto>>(),
    ))
}

async fn post_project(
    db: Extension<DatabaseConnection>,
    WithRejection(ValidatedJson(model), _): WithRejection<ValidatedJson<ProjectDto>, ApiError>,
) -> Result<Json<ProjectDto>, ApiError> {
    println!("Project(): '{}'", model.summary);
    let project = projects::ActiveModel {
        summary: Set(model.summary.to_owned()),
        deadline: Set(model.deadline.map(|d| d.date_naive())),
        user_id: Set(model.user_id),
        active: Set(model.active),
        ..Default::default()
    }
    .insert(&*db)
    .await?;
    Ok(Json(project.into()))
}

async fn put_project(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
    WithRejection(Json(update), _): WithRejection<Json<projects::Model>, ApiError>,
) -> impl IntoResponse {
    let original_result = Project::find_by_id(id).one(&*db).await?;
    match original_result {
        Some(original) => {
            let updated = projects::ActiveModel {
                id: Set(original.id),
                summary: Set(update.summary.to_owned()),
                deadline: Set(update.deadline.to_owned()),
                user_id: Set(update.user_id),
                active: Set(update.active),
            }
            .update(&*db)
            .await?;
            Ok(Json(updated))
        }
        None => Err(ApiError::new(
            StatusCode::NOT_FOUND,
            String::from("Not found"),
        )),
    }
}

async fn delete_project(
    db: Extension<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<u64>, ApiError>,
) -> impl IntoResponse {
    projects::ActiveModel {
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
