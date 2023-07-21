use crate::{api::project::ProjectApi, Route};
use shared::dtos::project::Project as ProjectDto;
use yew::prelude::*;
use yew_router::prelude::Link;

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
        let projects = self.list.iter().map(|ProjectDto { id, summary, deadline: _, user_id: _, active: _ }| {
            match id {
                Some(id) => html! {
                    <tr>
                        <th>
                            {id}
                        </th>
                        <td>
                            <Link<Route> classes={classes!("column", "is-full")} to={Route::Project { id: *id }}>
                                {summary.clone()}
                            </Link<Route>>
                        </td>
                    </tr>
                },
                None => html! { <></> }
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
                    <table class="table is-fullwidth is-hoverable">
                        <thead>
                            <tr>
                                <th>{ "Id" }</th>
                                <th>{ "Summary" }</th>
                            </tr>
                        </thead>
                        <tbody>
                        { for projects }
                        </tbody>
                    </table>
                </div>
            </div>
        }
    }
}
