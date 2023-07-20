use yew::prelude::*;

use crate::{api::project::ProjectApi, components::project_card::ProjectCard};
use shared::dtos::project::Project as ProjectDto;

pub enum Msg {
    FetchedProjects(Vec<ProjectDto>),
}

pub struct ProjectList {
    list: Vec<ProjectDto>,
}
impl Component for ProjectList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ProjectApi::fetch_all(ctx.link().callback(Msg::FetchedProjects));
        Self { list: Vec::new() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchedProjects(projects) => {
                self.list = projects;
            }
        }
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let projects = self.list.iter().map(|project| {
            html! {
                <div class="tile is-parent">
                    <div class="tile is-child">
                        <ProjectCard name={project.summary.clone()} id={project.id} />
                    </div>
                </div>
            }
        });

        html! {
            <div class="container">
                <section class="hero">
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">{ "Project list" }</h1>
                            <h2 class="subtitle">
                                { "Here you can see all projects of the application" }
                            </h2>
                        </div>
                    </div>
                </section>
                <p class="section py-0">
                    { "This is the list of all the created projects retrieved from the API in the background." }
                </p>
                <div class="section">
                    <div class="tile is-ancestor">
                        { for projects }
                    </div>
                </div>
            </div>
        }
    }
}
