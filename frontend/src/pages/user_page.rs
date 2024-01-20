use crate::{app_state::AppStateContext, route::Route, services::user_service::UserService};
use shared::dtos::user_dto::UserDto;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

pub enum Msg {
    ContextChanged(AppStateContext),
    FetchedUser(UserDto),
}

pub struct UserPage {
    user: UserDto,
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}
impl Component for UserPage {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(Msg::ContextChanged))
            .expect("context to be set");
        if app_state.identity.is_some() {
            UserService::fetch(ctx.props().id, ctx.link().callback(Msg::FetchedUser));
        } else {
            let navigator = ctx.link().navigator().unwrap();
            navigator.replace(&Route::Login);
        }
        Self {
            user: UserDto::default(),
            app_state,
            _listener,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        UserService::fetch(ctx.props().id, ctx.link().callback(Msg::FetchedUser));
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContextChanged(state) => {
                self.app_state = state;
            }
            Msg::FetchedUser(user) => {
                self.user = user;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self {
            user,
            app_state: _,
            _listener,
        } = self;

        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-light">
                            <p class="title">{ &user.name }</p>
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-info">
                                <div class="content">
                                    <p class="title">{ "Role" }</p>
                                    <div class="content">
                                        { &user.role.map_or(String::from(""), |r| r.to_string()) }
                                    </div>
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
