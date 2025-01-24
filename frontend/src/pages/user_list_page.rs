use crate::{
    app_state::AppStateContext,
    components::bulma::tables::{
        data_sources::user_data_source::UserDataSource, table::Table,
        table_data_source::ITableDataSource, table_head_data::TableHeadData,
    },
    services::user_service::UserService,
};
use implicit_clone::unsync::IString;
use shared::dtos::user_dto::{IUserDto, UserDto, UserField, UserValue};
use yew::prelude::*;

pub enum Msg {
    ContextChanged(AppStateContext),
    FetchedUsers(Vec<UserDto>),
    SortUsers(TableHeadData),
}

pub struct UserListPage {
    list: Vec<UserDto>,
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}
impl Component for UserListPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(Msg::ContextChanged))
            .expect("context to be set");
        UserListPage::init(&app_state, ctx);
        Self {
            list: Vec::new(),
            app_state,
            _listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContextChanged(state) => {
                self.app_state = state;
            }
            Msg::FetchedUsers(users) => {
                self.list = users;
            }
            Msg::SortUsers(sortdata) => {
                if self.app_state.identity.is_some() {
                    UserService::fetch_all(
                        self.app_state.identity.clone().unwrap().token.clone(),
                        None,
                        sortdata.sort.as_ref().map(|s| s.sort.clone()),
                        sortdata
                            .sort
                            .as_ref()
                            .map(|s| IString::from(s.order.to_string())),
                        ctx.link().callback(Msg::FetchedUsers),
                    )
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let datasource: ITableDataSource<UserField, IUserDto, UserValue> =
            UserDataSource::from(&self.list).into();

        let sorthandler = Some(ctx.link().callback(Msg::SortUsers));

        html! {
            <div class="container">
                <section class="hero">
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">{ "User list" }</h1>
                            <h2 class="subtitle">
                                { "Here you can see all users of the application" }
                            </h2>
                        </div>
                    </div>
                </section>
                <p class="section py-0">
                    { "This is the list of all the registered users of the application retrieved from the API in the background." }
                </p>
                <div class="section">
                    <div class="tile is-ancestor">
                        <Table<UserField, IUserDto, UserValue> {datasource} {sorthandler} />
                    </div>
                </div>
            </div>
        }
    }
}

impl UserListPage {
    fn init(app_state: &AppStateContext, ctx: &Context<Self>) {
        if app_state.identity.is_some() {
            UserService::fetch_all(
                app_state.identity.clone().unwrap().token.clone(),
                None,
                None,
                None,
                ctx.link().callback(Msg::FetchedUsers),
            );
        }
    }
}
