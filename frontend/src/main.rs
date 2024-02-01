use frontend::app_state::AppStateProvider;
use frontend::authenticator::Authenticator;
use frontend::components::app_modal::AppModal;
use frontend::components::navbar::Navbar;
use frontend::pages::{
    home_page::HomePage, login_page::LoginPage, page_not_found::PageNotFound,
    project_board_page::ProjectBoardPage, project_list_page::ProjectListPage,
    project_new_page::ProjectNewPage, project_page::ProjectPage,
    registration_page::RegistrationPage, ticket_new_page::TicketNewPage, ticket_page::TicketPage,
    user_list_page::UserListPage, user_page::UserPage,
};
use frontend::route::Route;
use frontend::theming::Theming;
use implicit_clone::unsync::IString;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <AppStateProvider>
                    <Authenticator>
                        <Theming>
                            <Navbar/>

                            <main>
                                <Switch<Route> render={switch}/>
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
                            <AppModal/>
                        </Theming>
                    </Authenticator>
                </AppStateProvider>
            </BrowserRouter>
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
            html! { <UserPage id={IString::from(id.to_string())} /> }
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
        Route::Login => {
            html! { <LoginPage />}
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
