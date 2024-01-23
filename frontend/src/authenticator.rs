use crate::{
    app_state::{AppState, AppStateContext},
    pages::page_not_found::PageNotFound,
    route::{Route, RouteSelector},
    services::auth_service::try_authenticate,
};
use yew::{prelude::*, Children};
use yew_router::scope_ext::RouterScopeExt;

#[derive(Debug, PartialEq, Properties)]
pub struct AuthenticatorProps {
    #[prop_or_default]
    pub children: Children,
}

pub enum AuthenticatorMsg {
    ContextChanged(AppStateContext),
}

pub struct Authenticator {
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}

impl Component for Authenticator {
    type Message = AuthenticatorMsg;
    type Properties = AuthenticatorProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(AuthenticatorMsg::ContextChanged))
            .expect("context to be set");
        if app_state.identity.is_none() {
            try_authenticate(app_state.clone(), Callback::noop());
        }
        Self {
            app_state,
            _listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AuthenticatorMsg::ContextChanged(state) => {
                log::debug!(
                    ">>> Identity: {}",
                    <std::option::Option<shared::dtos::login_dto::LoginDto> as Clone>::clone(
                        &state.identity
                    )
                    .map(|i| i.username)
                    .unwrap_or_default()
                );
                self.app_state = state;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let route_option = ctx.link().route::<Route>();
        match route_option.clone() {
            Some(route) => {
                if RouteSelector::is_public(route) || self.app_state.identity.is_some() {
                    html! { for ctx.props().children.iter() }
                } else {
                    AppState::update_referer(&self.app_state, route_option);
                    let navigator = ctx.link().navigator().unwrap();
                    navigator.replace(&Route::Login);
                    html! { <h1>{ "Unauthorized" }</h1> }
                }
            }
            None => {
                html! {
                    <PageNotFound/>
                }
            }
        }
    }
}
