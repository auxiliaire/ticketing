use crate::Route;
use crate::api::project::ProjectApi;
use crate::api::user::UserApi;
use crate::components::check_tag::CheckTag;
use crate::components::user_link::UserLink;
use frontend::api::ticket::TicketApi;
use implicit_clone::sync::IString;
use shared::dtos::project::Project as ProjectDto;
use shared::dtos::ticket::Ticket as TicketDto;
use shared::dtos::user::User as UserDto;
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

pub enum Msg {
    FetchedProject(ProjectDto),
    FetchedUser(UserDto),
    FetchedTickets(Vec<TicketDto>),
}

pub struct Project {
    project: ProjectDto,
    user: Option<(u64, IString)>,
    ticket_list: Vec<TicketDto>,
}
impl Component for Project {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ProjectApi::fetch(ctx.props().id, ctx.link().callback(Msg::FetchedProject));
        TicketApi::fetch_all(ctx.link().callback(Msg::FetchedTickets));
        Self {
            project: ProjectDto::default(),
            user: None,
            ticket_list: vec![],
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
            Msg::FetchedTickets(tickets) => {
                self.ticket_list = tickets;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self {
            project,
            user,
            ticket_list,
        } = self;

        let tickets = ticket_list.iter().map(|TicketDto { id, title, description: _, project_id: _, status, user_id: _ }| {
            match id {
                Some(id) => html! {
                    <tr>
                        <th>
                            {id}
                        </th>
                        <td>
                            //<Link<Route> classes={classes!("column", "is-full")} to={Route::Ticket { id: *id }}>
                                {title.clone()}
                            //</Link<Route>>
                        </td>
                        <td>
                            {status}
                        </td>
                    </tr>
                },
                None => html! { <></> }
            }
        });
        
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
                    <div class="tile">
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-light">
                                <div class="content">
                                    <p class="title">{ "Tickets" }</p>
                                    <table class="table is-fullwidth is-hoverable">
                                        <thead>
                                            <tr>
                                                <th>{ "Id" }</th>
                                                <th>{ "Title" }</th>
                                                <th>{ "Status" }</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                        { for tickets }
                                        </tbody>
                                    </table>
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
