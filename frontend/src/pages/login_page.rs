use crate::{
    app_state::{AppState, AppStateContext},
    components::forms::login_form::LoginForm,
    route::Route,
    services::auth_service::{try_authenticate, AuthService},
};
use shared::{api::error::error_response::ErrorResponse, dtos::login_dto::LoginDto};
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

pub enum LoginMsg {
    ContextChanged(AppStateContext),
    Submitted((LoginDto, Callback<ErrorResponse>)),
    LoggedIn(LoginDto),
    Authenticated(Option<String>),
}

pub struct LoginPage {
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}

impl Component for LoginPage {
    type Message = LoginMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(LoginMsg::ContextChanged))
            .expect("context to be set");
        if app_state.identity.is_some() {
            let referer = app_state.referer.clone().unwrap_or(Route::Home);
            let navigator = ctx.link().navigator().unwrap();
            navigator.replace(&referer);
        } else {
            try_authenticate(
                app_state.clone(),
                ctx.link().callback(LoginMsg::Authenticated),
            );
        }
        Self {
            app_state,
            _listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMsg::ContextChanged(state) => {
                self.app_state = state;
            }
            LoginMsg::Submitted((creds, callback_error)) => {
                log::debug!("Submitted: {}", creds.username);
                AuthService::authenticate(
                    creds,
                    ctx.link().callback(LoginMsg::LoggedIn),
                    callback_error,
                );
            }
            LoginMsg::LoggedIn(creds) => {
                log::debug!("Created: {}", creds.username);
                AppState::update_identity(&self.app_state, Some(creds));
            }
            LoginMsg::Authenticated(auth_res) => {
                if auth_res.is_some() {
                    log::debug!("Authenticated successfully!");
                    let referer = self.app_state.referer.clone().unwrap_or(Route::Home);
                    let navigator = ctx.link().navigator().unwrap();
                    navigator.replace(&referer);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <section class="hero">
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">{ "Login" }</h1>
                            <h2 class="subtitle">
                                { "Please provide credentials to log in to the application" }
                            </h2>
                        </div>
                    </div>
                </section>
                <div class="section">
                    <LoginForm onsubmit={ctx.link().callback(LoginMsg::Submitted)} />
                </div>
            </div>
        }
    }
}
