use crate::{components::forms::project_form::ProjectForm, Route};
use frontend::services::project_service::ProjectService;
use shared::{api::error::error_response::ErrorResponse, dtos::project_dto::ProjectDto};
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

pub enum ProjectMsg {
    Submitted((ProjectDto, Callback<ErrorResponse>)),
    Created(ProjectDto),
}

pub struct ProjectNewPage {}
impl Component for ProjectNewPage {
    type Message = ProjectMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProjectMsg::Submitted((project, callback_error)) => {
                log::debug!("Submitted: {}", project);
                ProjectService::create(
                    project,
                    ctx.link().callback(ProjectMsg::Created),
                    callback_error,
                );
            }
            ProjectMsg::Created(project) => {
                log::debug!("Created: {}", project);
                let navigator = ctx.link().navigator().unwrap();
                navigator.replace(&Route::Project {
                    id: project.id.unwrap(),
                });
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
                            <h1 class="title">{ "Create Project" }</h1>
                            <h2 class="subtitle">
                                { "Here you can create new projects" }
                            </h2>
                        </div>
                    </div>
                </section>
                <div class="section">
                    <ProjectForm onsubmit={ctx.link().callback(ProjectMsg::Submitted)} />
                </div>
            </div>
        }
    }
}
