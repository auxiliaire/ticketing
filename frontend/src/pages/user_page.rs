use crate::{
    app_state::{AppState, AppStateContext},
    components::forms::preferences_form::PreferencesForm,
    services::user_service::UserService,
};
use implicit_clone::unsync::IString;
use shared::{
    api::error::error_response::ErrorResponse,
    dtos::{identity::Identity, preferences_dto::PreferencesDto, user_dto::UserDto},
};
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: IString,
}

pub enum UserPageMsg {
    ContextChanged(AppStateContext),
    FetchedUser(UserDto),
    PreferencesSubmitted((PreferencesDto, Callback<ErrorResponse>)),
    PreferencesUpdated(PreferencesDto),
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserPageMsg::ContextChanged(state) => {
                self.app_state = state;
            }
            UserPageMsg::FetchedUser(user) => {
                self.user = user;
            }
            UserPageMsg::PreferencesSubmitted((prefs, callback_error)) => {
                if let Some(Identity { token, .. }) = &self.app_state.identity {
                    UserService::update_preferences(
                        token.to_owned(),
                        prefs,
                        ctx.link().callback(UserPageMsg::PreferencesUpdated),
                        callback_error,
                    );
                }
            }
            UserPageMsg::PreferencesUpdated(prefs) => {
                AppState::update_preferences(&self.app_state, Some(prefs));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                    <div class="section p-3">
                        <h1 class="title px-5">{ "Role" }</h1>
                        <h1 class="subtitle px-5">
                            <span class="tag is-info">{ &user.role.map_or(String::from(""), |r| r.to_string()) }</span>
                        </h1>
                    </div>
                    {
                        if self.is_own_profile() {
                            html! {
                                <div class="section p-3">
                                    <h1 class="title px-5">{ "Preferences" }</h1>
                                    <PreferencesForm onsubmit={ctx.link().callback(UserPageMsg::PreferencesSubmitted)} />
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
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

    fn is_own_profile(&self) -> bool {
        match (self.user.public_id, self.app_state.clone().identity.clone()) {
            (Some(profile_id), Some(Identity { userid, .. })) => profile_id.eq(&userid),
            _ => false,
        }
    }
}
