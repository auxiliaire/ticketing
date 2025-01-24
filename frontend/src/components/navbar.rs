use crate::{
    app_state::{AppState, AppStateContext},
    components::theme_switcher::ThemeSwitcher,
    route::Route,
    services::auth_service::REFRESH_TOKEN_KEY,
};
use yew::prelude::*;
use yew_router::prelude::*;

pub enum NavbarMsg {
    ContextChanged(AppStateContext),
    ToggleNavbar,
    DoLogout,
}

pub struct Navbar {
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}

impl Component for Navbar {
    type Message = NavbarMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(NavbarMsg::ContextChanged))
            .expect("context to be set");
        Self {
            app_state,
            _listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavbarMsg::ContextChanged(state) => {
                self.app_state = state;
            }
            NavbarMsg::ToggleNavbar => AppState::toggle_navbar(&self.app_state),
            NavbarMsg::DoLogout => {
                let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
                storage.delete(REFRESH_TOKEN_KEY).unwrap();

                AppState::update_identity(&self.app_state, None);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let active_class = if !self.app_state.navbar_active {
            "is-active"
        } else {
            ""
        };

        let on_logout_click = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            NavbarMsg::DoLogout
        });

        html! {
            <nav class="navbar is-link" role="navigation" aria-label="main navigation">
                <div class="navbar-brand is-size-4">
                    <div class="navbar-item pr-0">
                        <i class="fa-solid fa-tag has-text-primary"></i>
                    </div>
                    <h1 class="navbar-item is-size-3">
                        { "Ticketing in Rust" }
                    </h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={ctx.link().callback(|_| NavbarMsg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        {
                            if self.app_state.identity.is_some() {
                                html! {
                                    <>
                                        /*<Link<Route> classes={classes!("navbar-item")} to={Route::Tickets}>
                                            { "Tickets" }
                                        </Link<Route>>*/

                                        <div class="navbar-item has-dropdown is-hoverable">
                                            <div class="navbar-link">
                                                { "Options" }
                                            </div>
                                            <div class="navbar-dropdown">
                                                <Link<Route> classes={classes!("navbar-item")} to={Route::Users}>
                                                    { "List of users" }
                                                </Link<Route>>
                                                <Link<Route> classes={classes!("navbar-item")} to={Route::Projects}>
                                                    { "List of projects" }
                                                </Link<Route>>
                                                    <Link<Route> classes={classes!("navbar-item")} to={Route::ProjectNew}>
                                                    { "Create new project" }
                                                </Link<Route>>
                                            </div>
                                        </div>

                                        <div class="navbar-item">
                                            <Link<Route> classes={classes!("button", "is-info", "is-light")} to={Route::TicketNew}>
                                                { "Create" }
                                            </Link<Route>>
                                        </div>
                                    </>
                                }
                            } else { html! { <></> } }
                        }
                    </div>
                        <div class="navbar-end">
                            <div class="navbar-item is-hoverable">
                                <ThemeSwitcher/>
                            </div>
                        {
                            if self.app_state.identity.is_some() {
                                html! {
                                    <div class="navbar-item has-dropdown is-hoverable">
                                        <div class="navbar-link">
                                            <i class="fa-solid fa-user"></i>
                                        </div>
                                        <div class="navbar-dropdown is-right">
                                            <Link<Route> classes={classes!("navbar-item")} to={Route::User { id: <std::option::Option<shared::dtos::identity::Identity> as Clone>::clone(&self.app_state.identity).unwrap().userid }}>
                                                { "Profile" }
                                            </Link<Route>>
                                            <a href="#" class="navbar-item" onclick={on_logout_click}>
                                                    { "Logout" }
                                            </a>
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {
                                    <>
                                        <div class="navbar-item pr-2">
                                            <Link<Route> classes={classes!("button", "is-primary", "is-light")} to={Route::Registration}>
                                                { "Register" }
                                        </Link<Route>>
                                        </div>
                                        <div class="navbar-item pl-2">
                                            <Link<Route> classes={classes!("button", "is-info", "is-light")} to={Route::Login}>
                                                { "Login" }
                                            </Link<Route>>
                                        </div>
                                    </>
                                }
                            }
                        }
                        </div>
                </div>
            </nav>
        }
    }
}
