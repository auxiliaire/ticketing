use crate::api::project::ProjectApi;
use crate::api::{ticket::TicketApi, user::UserApi};
use crate::components::button_link::{ButtonLink, ButtonLinkData};
use crate::Route;
use implicit_clone::sync::IString;
use shared::dtos::project::Project as ProjectDto;
use shared::dtos::ticket::Ticket as TicketDto;
use shared::dtos::user::User as UserDto;
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

pub enum Msg {
    FetchedTicket(TicketDto),
    FetchedProject(ProjectDto),
    FetchedUser(UserDto),
}

pub struct Ticket {
    ticket: TicketDto,
    project: Option<ButtonLinkData<Route>>,
    user: Option<ButtonLinkData<Route>>,
}
impl Component for Ticket {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        TicketApi::fetch(ctx.props().id, ctx.link().callback(Msg::FetchedTicket));
        Self {
            ticket: TicketDto::default(),
            project: None,
            user: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        TicketApi::fetch(ctx.props().id, ctx.link().callback(Msg::FetchedTicket));
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchedTicket(ticket) => {
                self.ticket = ticket;
                if let Some(project_id) = self.ticket.project_id {
                    ProjectApi::fetch(project_id, ctx.link().callback(Msg::FetchedProject));
                }
                if let Some(user_id) = self.ticket.user_id {
                    UserApi::fetch(user_id, ctx.link().callback(Msg::FetchedUser));
                }
            }
            Msg::FetchedProject(project) => {
                self.project = Some(ButtonLinkData {
                    label: IString::from(project.summary),
                    to: Route::Project {
                        id: project.id.unwrap(),
                    },
                });
            }
            Msg::FetchedUser(user) => {
                self.user = Some(ButtonLinkData {
                    label: IString::from(user.name),
                    to: Route::User {
                        id: user.id.unwrap(),
                    },
                });
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self {
            ticket,
            project,
            user,
        } = self;

        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-light">
                            <p class="title">{ &ticket.title }</p>
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-light">
                                <div class="content">
                                    <p class="title">{ "Details" }</p>
                                    <div class="columns">
                                        <div class="column is-one-quarter"><h5 class="title is-5">{ "Description" }</h5></div>
                                        <div class="column">{ &ticket.description }</div>
                                    </div>
                                    <div class="columns">
                                        <div class="column is-one-quarter"><h5 class="title is-5">{ "Project" }</h5></div>
                                        <div class="column">
                                            <ButtonLink<Route> data={project.clone()} />
                                        </div>
                                    </div>
                                    <div class="columns">
                                        <div class="column is-one-quarter"><h5 class="title is-5">{ "Status" }</h5></div>
                                        <div class="column">{ &ticket.status.to_string() }</div>
                                    </div>
                                    <div class="columns">
                                        <div class="column is-one-quarter"><h5 class="title is-5">{ "Assigned to" }</h5></div>
                                        <div class="column">
                                            <ButtonLink<Route> data={user.clone()} />
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
