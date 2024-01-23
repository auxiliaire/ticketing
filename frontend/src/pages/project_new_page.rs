use crate::app_state::AppStateContext;
use crate::services::project_service::ProjectService;
use crate::{components::forms::project_form::ProjectForm, route::Route};
use shared::dtos::login_dto::LoginDto;
use shared::{api::error::error_response::ErrorResponse, dtos::project_dto::ProjectDto};
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

pub enum ProjectMsg {
    ContextChanged(AppStateContext),
    Submitted((ProjectDto, Callback<ErrorResponse>)),
    Created(ProjectDto),
}

pub struct ProjectNewPage {
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}

impl Component for ProjectNewPage {
    type Message = ProjectMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(ProjectMsg::ContextChanged))
            .expect("context to be set");
        if app_state.identity.is_none() {
            let navigator = ctx.link().navigator().unwrap();
            navigator.replace(&Route::Login);
        }
        Self {
            app_state,
            _listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProjectMsg::ContextChanged(state) => {
                self.app_state = state;
            }
            ProjectMsg::Submitted((project, callback_error)) => {
                if let Some(LoginDto { token, .. }) = &self.app_state.identity {
                    ProjectService::create(
                        token.to_string(),
                        project,
                        ctx.link().callback(ProjectMsg::Created),
                        callback_error,
                    );
                }
            }
            ProjectMsg::Created(project) => {
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
