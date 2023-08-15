use crate::{
    components::bulma::tables::{
        data_sources::project_data_source::ProjectDataSource, table::Table,
        table_data_source::ITableDataSource, table_head_data::TableHeadData,
    },
    services::project_service::ProjectService,
    Route,
};
use implicit_clone::unsync::IString;
use shared::dtos::project_dto::{IProjectDto, ProjectDto, ProjectField, ProjectValue};
use yew::prelude::*;
use yew_router::prelude::Link;

pub enum Msg {
    FetchedProjects(Vec<ProjectDto>),
    SortProjects(TableHeadData),
}

pub struct ProjectListPage {
    list: Vec<ProjectDto>,
}
impl Component for ProjectListPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ProjectService::fetch_all(None, None, ctx.link().callback(Msg::FetchedProjects));
        Self { list: Vec::new() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchedProjects(projects) => {
                self.list = projects;
            }
            Msg::SortProjects(sortdata) => ProjectService::fetch_all(
                sortdata.sort.as_ref().map(|s| s.sort.clone()),
                sortdata
                    .sort
                    .as_ref()
                    .map(|s| IString::from(s.order.to_string())),
                ctx.link().callback(Msg::FetchedProjects),
            ),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let datasource: ITableDataSource<ProjectField, IProjectDto, ProjectValue> =
            ProjectDataSource::from(&self.list).into();

        let sorthandler = Some(ctx.link().callback(Msg::SortProjects));

        html! {
            <div class="container">
                <section class="hero">
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">{ "Project list" }</h1>
                            <h2 class="subtitle">
                                { "Here you can see all projects of the application" }
                            </h2>
                        </div>
                    </div>
                </section>
                <p class="section py-0">
                    { "This is the list of all the created projects retrieved from the API in the background." }
                </p>
                <div class="section">
                    <Table<ProjectField, IProjectDto, ProjectValue> {datasource} {sorthandler} />
                </div>
                <div class="section pt-0">
                    <div class="field is-grouped">
                        <p class="control">
                            <Link<Route> classes={classes!("button", "is-full")} to={Route::ProjectNew}>
                                <span class="icon is-small">
                                    <i class="fas fa-plus"></i>
                                </span>
                                <span>{ "Start a new project" }</span>
                            </Link<Route>>
                        </p>
                    </div>
                </div>
            </div>
        }
    }
}
