use crate::pages::project_board_page::ProjectBoardPage;
use crate::pages::project_list_page::ProjectListPage;
use crate::pages::project_new_page::ProjectNewPage;
use crate::pages::project_page::ProjectPage;
use crate::pages::registration_page::RegistrationPage;
use crate::pages::ticket_new_page::TicketNewPage;
use crate::pages::ticket_page::TicketPage;
use pages::home_page::HomePage;
use pages::page_not_found::PageNotFound;
use pages::user_list_page::UserListPage;
use pages::user_page::UserPage;
use std::rc::Rc;
use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod services;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/projects/:id/board")]
    ProjectBoard { id: u64 },
    #[at("/projects/new")]
    ProjectNew,
    #[at("/projects/:id")]
    Project { id: u64 },
    #[at("/projects")]
    Projects,
    #[at("/register")]
    Registration,
    #[at("/users/:id")]
    User { id: u64 },
    #[at("/users")]
    Users,
    #[at("/tickets/new")]
    TicketNew,
    #[at("/tickets/:id")]
    Ticket { id: u64 },
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum AppMsg {
    ToggleNavbar,
    UpdateDialog(Rc<Dialog>),
    CloseDialog,
}

#[derive(Clone, Default, PartialEq)]
pub struct Dialog {
    active: bool,
    content: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct AppState {
    update_dialog: Callback<Rc<Dialog>>,
    close_dialog: Callback<()>,
    navbar_active: bool,
}

pub struct App {
    state: Rc<AppState>,
    dialog: Rc<Dialog>,
}
impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let update_dialog = ctx.link().callback(AppMsg::UpdateDialog);
        let close_dialog = ctx.link().callback(|_| AppMsg::CloseDialog);
        let state = Rc::new(AppState {
            navbar_active: false,
            update_dialog,
            close_dialog,
        });
        Self {
            state,
            dialog: Rc::new(Dialog::default()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::ToggleNavbar => {
                let shared_state = Rc::make_mut(&mut self.state);
                shared_state.navbar_active = !shared_state.navbar_active;
            }
            AppMsg::UpdateDialog(dialog) => {
                self.dialog = dialog;
            }
            AppMsg::CloseDialog => {
                self.dialog = Rc::new(Dialog::default());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let modal_active = match self.dialog.active {
            true => "is-active",
            false => "",
        };

        html! {
            <BrowserRouter>
                <ContextProvider<Rc<AppState>> context={self.state.clone()}>
                    { self.view_nav(ctx.link()) }

                    <main>
                        <Switch<Route> render={switch} />
                    </main>
                    <footer class="footer">
                        <div class="content has-text-centered">
                            { "Powered by " }
                            <a href="https://crates.io/crates/axum">{ "Axum" }</a>
                            { ", " }
                            <a href="https://www.sea-ql.org/SeaORM/">{ "SeaORM" }</a>
                            { ", " }
                            <a href="https://yew.rs">{ "Yew" }</a>
                            { " and " }
                            <a href="https://bulma.io">{ "Bulma" }</a>
                        </div>
                    </footer>
                    <div id="app-modal" class={classes!("modal", modal_active)}>
                        <div class="modal-background"></div>
                        <div class="modal-card">
                            { self.dialog.clone().content.clone() }
                        </div>
                    </div>
                </ContextProvider<Rc<AppState>>>
            </BrowserRouter>
        }
    }
}
impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let active_class = if !self.state.navbar_active {
            "is-active"
        } else {
            ""
        };

        html! {
            <nav class="navbar is-link" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Ticketing in Rust" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| AppMsg::ToggleNavbar)}
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
                        /*<Link<Route> classes={classes!("navbar-item")} to={Route::Tickets}>
                            { "Tickets" }
                        </Link<Route>>*/
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Registration}>
                            { "Register" }
                        </Link<Route>>

                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "More" }
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
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::ProjectBoard { id } => {
            html! { <ProjectBoardPage id={id} /> }
        }
        Route::ProjectNew => {
            html! { <ProjectNewPage /> }
        }
        Route::Project { id } => {
            html! { <ProjectPage id={id} /> }
        }
        Route::Projects => {
            html! { <ProjectListPage /> }
        }
        Route::Registration => {
            html! { <RegistrationPage /> }
        }
        Route::User { id } => {
            html! { <UserPage id={id} /> }
        }
        Route::Users => {
            html! { <UserListPage /> }
        }
        Route::TicketNew => {
            html! { <TicketNewPage /> }
        }
        Route::Ticket { id } => {
            html! { <TicketPage id={id} /> }
        }
        Route::Home => {
            html! { <HomePage /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
