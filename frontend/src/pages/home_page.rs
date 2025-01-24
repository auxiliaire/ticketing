use crate::{
    app_state::{AppState, AppStateContext},
    components::{
        dialogs::form_dialog::FormDialog,
        forms::{login_form::LoginForm, registration_form::RegistrationForm},
    },
    dialog::Dialog,
    route::Route,
    services::{auth_service::AuthService, project_service::ProjectService},
};
use shared::{
    api::error::error_response::ErrorResponse,
    dtos::{identity::Identity, login_dto::LoginDto, project_dto::ProjectDto, user_dto::UserDto},
};
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::Link;

pub enum HomeMsg {
    FetchedProjects(Vec<ProjectDto>),
    ContextChanged(AppStateContext),
    OpenRegistrationDialog,
    OpenLoginDialog,
    SubmittedLoginForm((LoginDto, Callback<ErrorResponse>)),
    SubmittedRegistrationForm((UserDto, Callback<ErrorResponse>)),
    LoggedIn(Identity),
}

pub struct HomePage {
    list: Vec<ProjectDto>,
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}

impl Component for HomePage {
    type Message = HomeMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(HomeMsg::ContextChanged))
            .expect("context to be set");
        HomePage::init(&app_state, ctx);
        Self {
            list: Vec::with_capacity(3),
            app_state,
            _listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::FetchedProjects(projects) => {
                self.list = projects;
            }
            HomeMsg::ContextChanged(state) => {
                self.app_state = state;
                HomePage::init(&self.app_state, ctx);
            }
            HomeMsg::OpenLoginDialog => {
                let dialog = Rc::new(Dialog {
                    active: true,
                    content: html! {
                        <FormDialog title="Please provide credentials">
                            <LoginForm onsubmit={ctx.link().callback(HomeMsg::SubmittedLoginForm)} />
                        </FormDialog>
                    },
                });
                AppState::update_dialog(&self.app_state, dialog);
            }
            HomeMsg::OpenRegistrationDialog => {
                let dialog = Rc::new(Dialog {
                    active: true,
                    content: html! {
                        <FormDialog title="Fill out to register">
                            <RegistrationForm onsubmit={ctx.link().callback(HomeMsg::SubmittedRegistrationForm)} />
                        </FormDialog>
                    },
                });
                AppState::update_dialog(&self.app_state, dialog);
            }
            HomeMsg::SubmittedLoginForm((creds, callback_error)) => {
                log::debug!("Submitted login creds for {}", creds.username);
                AuthService::authenticate(
                    creds,
                    ctx.link().callback(HomeMsg::LoggedIn),
                    callback_error,
                );
            }
            HomeMsg::SubmittedRegistrationForm(_) => todo!(),
            HomeMsg::LoggedIn(identity) => {
                log::debug!("Logged in {}", identity);
                AppState::update_identity_and_close_dialog(&self.app_state, Some(identity));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "Welcome" }</h1>
                        <h2 class="subtitle">{ "Here you can manage your IT project" }</h2>
                    </div>
                </div>

                <div class="tile is-parent container">
                    {
                        if self.app_state.identity.is_some() {
                            self.view_info_tiles()
                        } else {
                            self.view_unauthorized(ctx)
                        }
                    }
                </div>
            </div>
        }
    }
}

impl HomePage {
    fn init(app_state: &AppStateContext, ctx: &Context<Self>) {
        if app_state.identity.is_some() {
            ProjectService::fetch_latest(
                app_state.identity.clone().unwrap().token.clone(),
                ctx.link().callback(HomeMsg::FetchedProjects),
            );
        }
    }

    fn view_info_tiles(&self) -> Html {
        let projects = self.list.iter().map(|ProjectDto { id, summary, deadline: _, user_id: _, active: _ }| {
            match id {
                Some(id) => html! {
                    <tr>
                        <td>
                            <Link<Route> classes={classes!("column", "is-full", "pl-0", "pt-0")} to={Route::Project { id: *id }}>
                                {summary.clone()}
                            </Link<Route>>
                        </td>
                    </tr>
                },
                None => html! { <></> }
            }
        });
        html! {
            <>
                <div class="tile is-parent">
                    <div class="tile is-child box" style="display: flex; flex-direction: column">
                        <p class="title">{ "Start a New Project" }</p>
                        <p class="subtitle">{ "Everything you need to know!" }</p>

                        <p>{r#"
                        Creating a new project has never been easier. Just click the button below,
                        fill out the details, and you can add new user stories right away.
                        "#}
                        </p>
                        <div class="columns is-mobile is-centered is-vcentered mt-0 mb-0 ml-0 mr-0" style="flex-grow: 4">
                            <Link<Route> classes={classes!("button", "is-info")} to={Route::ProjectNew}>
                                { "Create a New Project" }
                            </Link<Route>>
                        </div>
                    </div>
                </div>

                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "Continue working on an existing one" }</p>
                        <p class="subtitle">{ "The last couple of projects" }</p>

                        <div class="content">
                            {
                                if self.list.is_empty() {
                                    html! {
                                        <em>{ "No projects yet" }</em>
                                    }
                                }
                                else
                                {
                                    html! {
                                        <table class="table is-fullwidth is-hoverable">
                                            <tbody>
                                            { for projects }
                                            </tbody>
                                        </table>
                                    }
                                }
                            }
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn view_unauthorized(&self, ctx: &Context<Self>) -> Html {
        let on_register_click = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            HomeMsg::OpenRegistrationDialog
        });
        let on_login_click = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            HomeMsg::OpenLoginDialog
        });
        html! {
            <div class="tile is-parent">
                <div class="tile is-child box" style="display: flex; flex-direction: column">
                    <p class="title">{ "Start a New Project" }</p>
                    <p class="subtitle">{ "Get started today!" }</p>

                    <p>
                        { "In order to gain access to our ticket management application, please " }
                        <a href="#" onclick={on_register_click}>
                            { "register" }
                        </a>
                        { " or " }
                        <a href="#" onclick={on_login_click}>
                            { "log in" }
                        </a>
                        { "." }
                    </p>
                </div>
            </div>
        }
    }
}
