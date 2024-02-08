use crate::{
    app_state::{AppState, AppStateContext},
    helpers::storage_helper::store_in_storage,
    pages::page_not_found::PageNotFound,
    route::{Route, RouteSelector},
    services::auth_service::{AuthService, REFRESH_TOKEN_KEY},
};
use shared::dtos::identity::Identity;
use yew::{prelude::*, Children};
use yew_router::scope_ext::RouterScopeExt;

#[derive(Debug, PartialEq, Properties)]
pub struct AuthenticatorProps {
    #[prop_or_default]
    pub children: Children,
}

pub enum AuthenticatorMsg {
    ContextChanged(AppStateContext),
    IdentityVerified(Identity),
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
        Authenticator::init(&app_state, ctx);
        Self {
            app_state,
            _listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AuthenticatorMsg::ContextChanged(state) => {
                log::debug!(
                    ">>> Identity: {}",
                    <std::option::Option<shared::dtos::identity::Identity> as Clone>::clone(
                        &state.identity
                    )
                    .map(|i| i.userid)
                    .unwrap_or_default()
                );
                self.app_state = state;
            }
            AuthenticatorMsg::IdentityVerified(identity) => {
                log::debug!("Identity verified");
                AppState::update_identity(&self.app_state, Some(identity));
                let navigator = ctx.link().navigator().unwrap();
                navigator.replace(&Route::Home);
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

impl Authenticator {
    fn init(app_state: &AppStateContext, ctx: &Context<Self>) {
        log::debug!("Authenticator::init");
        if app_state.identity.is_none() {
            AuthService::try_authenticate(app_state.clone(), Callback::noop());
        }
        let route_option = ctx.link().route::<Route>();
        if let Some(Route::Verify { token }) = route_option {
            store_in_storage(REFRESH_TOKEN_KEY.to_string(), token.to_string());
            AuthService::fetch_jwt(
                token.to_string(),
                ctx.link().callback(AuthenticatorMsg::IdentityVerified),
                Callback::noop(),
            );
        };
    }
}
