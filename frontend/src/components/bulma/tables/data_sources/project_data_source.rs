use crate::{
    components::{
        bulma::tables::{
            composite_cell_data::CompositeCellData,
            table_data_source::{ITableDataSource, TableDataSource},
        },
        check_tag::CheckTag,
    },
    Route,
};
use implicit_clone::unsync::{IArray, IString};
use shared::dtos::project_dto::{IProjectDto, ProjectDto, ProjectField, ProjectValue};
use std::rc::Rc;
use yew::{classes, html, Callback};
use yew_router::prelude::Link;

pub struct ProjectDataSource(ITableDataSource<ProjectField, IProjectDto, ProjectValue>);

impl From<&Vec<ProjectDto>> for ProjectDataSource {
    fn from(source: &Vec<ProjectDto>) -> Self {
        Self(Rc::new(TableDataSource {
            empty_label: IString::from("There are no projects yet"),
            data: IArray::from(
                source
                    .iter()
                    .map(|project| Rc::new(project.clone()))
                    .collect::<Vec<IProjectDto>>(),
            ),
            has_column_head: true,
            has_row_head: true,
            cellrenderer: Callback::from(
                |celldata: CompositeCellData<ProjectField, IProjectDto>| match celldata.data.id {
                    Some(id) => match celldata.column {
                        ProjectField::Id => Some(html! {
                            {id}
                        }),
                        ProjectField::Summary => Some(html! {
                            <Link<Route> classes={classes!("column", "is-full", "pl-0", "pt-0", "pb-0")} to={Route::ProjectBoard { id }}>
                                {celldata.data.summary.clone()}
                            </Link<Route>>
                        }),
                        ProjectField::Deadline => Some(html! {
                            { celldata.data.deadline.map_or(String::from("-"), |d| d.format("%F").to_string()) }
                        }),
                        ProjectField::Active => Some(html! {
                            <CheckTag checked={celldata.data.active == 1} />
                        }),
                    },
                    None => None,
                },
            ),
            ..Default::default()
        }))
    }
}

impl From<ProjectDataSource> for ITableDataSource<ProjectField, IProjectDto, ProjectValue> {
    fn from(val: ProjectDataSource) -> Self {
        val.0
    }
}
