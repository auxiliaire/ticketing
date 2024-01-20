use crate::{
    app_state::AppStateContext,
    components::bulma::{
        pagination::Pagination,
        tables::{
            data_sources::project_data_source::ProjectDataSource, table::Table,
            table_data_source::ITableDataSource, table_head_data::TableHeadData,
        },
    },
    route::Route,
    services::project_service::ProjectService,
};
use implicit_clone::unsync::IString;
use shared::dtos::{
    page::Page,
    project_dto::{IProjectDto, ProjectDto, ProjectField, ProjectValue},
};
use yew::prelude::*;
use yew_router::{prelude::Link, scope_ext::RouterScopeExt};

const DEFAULT_LIMIT: u64 = 5;
const DEFAULT_OFFSET: u64 = 0;

pub enum Msg {
    ContextChanged(AppStateContext),
    FetchedProjects(Page<ProjectDto>),
    SortProjects(TableHeadData),
    UpdateOffset(u64),
}

pub struct ProjectListPage {
    total: i64,
    list: Vec<ProjectDto>,
    sort: Option<IString>,
    order: Option<IString>,
    limit: u64,
    offset: u64,
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}
impl Component for ProjectListPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(Msg::ContextChanged))
            .expect("context to be set");
        let sort = None;
        let order = None;
        let limit = DEFAULT_LIMIT;
        let offset = DEFAULT_OFFSET;
        if app_state.identity.is_some() {
            ProjectService::fetch_all(
                sort.clone(),
                order.clone(),
                Some(limit),
                Some(offset),
                ctx.link().callback(Msg::FetchedProjects),
            );
        } else {
            let navigator = ctx.link().navigator().unwrap();
            navigator.replace(&Route::Login);
        }
        Self {
            total: 0,
            list: Vec::new(),
            sort,
            order,
            limit,
            offset,
            app_state,
            _listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContextChanged(state) => {
                self.app_state = state;
            }
            Msg::FetchedProjects(project_page) => {
                self.total = project_page.total;
                self.list = project_page.list;
                self.limit = project_page.limit;
                self.offset = project_page.offset;
            }
            Msg::SortProjects(sortdata) => {
                self.sort = sortdata.sort.as_ref().map(|s| s.sort.clone());
                self.order = sortdata
                    .sort
                    .as_ref()
                    .map(|s| IString::from(s.order.to_string()));
                ProjectService::fetch_all(
                    self.sort.clone(),
                    self.order.clone(),
                    Some(self.limit),
                    Some(self.offset),
                    ctx.link().callback(Msg::FetchedProjects),
                )
            }
            Msg::UpdateOffset(offset) => {
                self.offset = offset;
                ProjectService::fetch_all(
                    self.sort.clone(),
                    self.order.clone(),
                    Some(self.limit),
                    Some(self.offset),
                    ctx.link().callback(Msg::FetchedProjects),
                )
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let datasource: ITableDataSource<ProjectField, IProjectDto, ProjectValue> =
            ProjectDataSource::from(&self.list).into();

        let sorthandler = Some(ctx.link().callback(Msg::SortProjects));
        let paginghandler = ctx.link().callback(Msg::UpdateOffset);

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
                    <Pagination total={self.total} offset={self.offset} limit={self.limit} {paginghandler} />
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
