use crate::{services::project_service::ProjectService, Route};
use shared::dtos::project::Project as ProjectDto;
use yew::prelude::*;
use yew_router::prelude::Link;

pub enum HomeMsg {
    FetchedProjects(Vec<ProjectDto>),
}

pub struct Home {
    list: Vec<ProjectDto>,
}
impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ProjectService::fetch_latest(ctx.link().callback(HomeMsg::FetchedProjects));
        Self {
            list: Vec::with_capacity(3),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::FetchedProjects(projects) => {
                self.list = projects;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "Welcome User" }</h1>
                        <h2 class="subtitle">{ "Here you can manage your IT project" }</h2>
                    </div>
                </div>

                <div class="tile is-parent container">
                    { self.view_info_tiles() }
                </div>
            </div>
        }
    }
}
impl Home {
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
}
