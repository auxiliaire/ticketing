use crate::api::project::ProjectApi;
use crate::api::user::UserApi;
use crate::components::check_tag::CheckTag;
use crate::components::user_link::UserLink;
use implicit_clone::sync::IString;
use shared::dtos::project::Project as ProjectDto;
use shared::dtos::user::User as UserDto;
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

pub enum Msg {
    FetchedProject(ProjectDto),
    FetchedUser(UserDto),
}

pub struct Project {
    project: ProjectDto,
    user: Option<(u64, IString)>,
}
impl Component for Project {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ProjectApi::fetch(ctx.props().id, ctx.link().callback(Msg::FetchedProject));
        Self {
            project: ProjectDto::default(),
            user: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        ProjectApi::fetch(ctx.props().id, ctx.link().callback(Msg::FetchedProject));
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchedProject(project) => {
                self.project = project;
                UserApi::fetch(self.project.user_id, ctx.link().callback(Msg::FetchedUser));
            }
            Msg::FetchedUser(user) => {
                self.user = Some((user.id.unwrap(), IString::from(user.name)));
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { project, user } = self;

        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-light">
                            <p class="title">{ &project.summary }</p>
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-light">
                                <div class="content">
                                    <p class="title">{ "Details" }</p>
                                    <div class="columns">
                                        <div class="column is-one-quarter"><h5 class="title is-5">{ "Deadline" }</h5></div>
                                        <div class="column">{ &project.deadline.map_or(String::from("-"), |d| d.format("%F").to_string()) }</div>
                                    </div>
                                    <div class="columns">
                                        <div class="column is-one-quarter"><h5 class="title is-5">{ "Created by" }</h5></div>
                                        <div class="column">
                                            <UserLink {user} />
                                        </div>
                                    </div>
                                    <div class="columns">
                                        <div class="column is-one-quarter"><h5 class="title is-5">{ "Active" }</h5></div>
                                        <div class="column">
                                            <CheckTag checked={project.active == 1} />
                                        </div>
                                    </div>
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
