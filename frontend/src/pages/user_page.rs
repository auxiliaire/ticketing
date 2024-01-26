use crate::{app_state::AppStateContext, services::user_service::UserService};
use implicit_clone::unsync::IString;
use shared::dtos::{identity::Identity, user_dto::UserDto};
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: IString,
}

pub enum UserPageMsg {
    ContextChanged(AppStateContext),
    FetchedUser(UserDto),
}

pub struct UserPage {
    user: UserDto,
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}
impl Component for UserPage {
    type Message = UserPageMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(UserPageMsg::ContextChanged))
            .expect("context to be set");
        UserPage::init(&app_state, ctx);
        Self {
            user: UserDto::default(),
            app_state,
            _listener,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        UserPage::init(&self.app_state, ctx);
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserPageMsg::ContextChanged(state) => {
                self.app_state = state;
            }
            UserPageMsg::FetchedUser(user) => {
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
                            <p class="subtitle">{ &user.username.to_string() }</p>
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

impl UserPage {
    fn init(app_state: &AppStateContext, ctx: &Context<Self>) {
        if let Some(Identity { token, .. }) = &app_state.identity {
            UserService::fetch(
                token.to_string(),
                Uuid::parse_str(ctx.props().id.as_str()).unwrap(),
                ctx.link().callback(UserPageMsg::FetchedUser),
            );
        }
    }
}
