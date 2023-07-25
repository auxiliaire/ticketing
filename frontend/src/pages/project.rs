use crate::api::project::ProjectApi;
use crate::api::user::UserApi;
use crate::components::check_tag::CheckTag;
use crate::components::dialogs::form_dialog::FormDialog;
use crate::components::dialogs::select_dialog::SelectDialog;
use crate::components::forms::ticket_form::TicketForm;
use crate::components::option_data::OptionData;
use crate::components::user_link::UserLink;
use crate::{AppState, Dialog};
use frontend::api::ticket::TicketApi;
use implicit_clone::sync::{IArray, IString};
use shared::dtos::project::Project as ProjectDto;
use shared::dtos::ticket::Ticket as TicketDto;
use shared::dtos::user::User as UserDto;
use std::rc::Rc;
use yew::prelude::*;

impl OptionData for TicketDto {
    fn get_key(&self) -> implicit_clone::unsync::IString {
        implicit_clone::unsync::IString::from(format!("{}", self.id.unwrap()))
    }

    fn get_label(&self) -> implicit_clone::unsync::IString {
        implicit_clone::unsync::IString::from(format!("{}", self.title.as_str()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

pub enum Msg {
    FetchedProject(ProjectDto),
    FetchedUser(UserDto),
    FetchedTickets(Vec<TicketDto>),
    ContextChanged(Rc<AppState>),
    OpenSelectDialog(),
    OpenFormDialog(),
    SelectedTickets(IArray<u64>),
    SubmittedForm(),
}

pub struct Project {
    project: ProjectDto,
    user: Option<(u64, IString)>,
    ticket_list: Vec<TicketDto>,
    app_state: Rc<AppState>,
    _listener: ContextHandle<Rc<AppState>>,
}
impl Component for Project {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ProjectApi::fetch(ctx.props().id, ctx.link().callback(Msg::FetchedProject));
        ProjectApi::fetch_assigned_tickets(
            ctx.props().id,
            ctx.link().callback(Msg::FetchedTickets),
        );
        let (app_state, _listener) = ctx
            .link()
            .context::<Rc<AppState>>(ctx.link().callback(Msg::ContextChanged))
            .expect("context to be set");
        Self {
            project: ProjectDto::default(),
            user: None,
            ticket_list: vec![],
            app_state,
            _listener,
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
            Msg::ContextChanged(state) => {
                self.app_state = state;
            }
            Msg::OpenSelectDialog() => {
                let optionsapi: Callback<Callback<Vec<TicketDto>>> =
                    Callback::from(TicketApi::fetch_unassigned);
                let onselect: Callback<IArray<u64>> = ctx.link().callback(Msg::SelectedTickets);
                let dialog = Rc::new(Dialog {
                    active: true,
                    content: html! {
                        <SelectDialog<u64, TicketDto> {optionsapi} {onselect} />
                    },
                });
                self.app_state.update_dialog.emit(dialog);
            }
            Msg::OpenFormDialog() => {
                let on_submit = |_| Msg::SubmittedForm();
                let dialog = Rc::new(Dialog {
                    active: true,
                    content: html! {
                        <FormDialog>
                            <TicketForm isdialog={true} projectid={ctx.props().id} onsubmit={ctx.link().callback(on_submit)} />
                        </FormDialog>
                    },
                });
                self.app_state.update_dialog.emit(dialog);
            }
            Msg::SelectedTickets(tickets) => {
                let callback = ctx.link().callback(Msg::FetchedTickets);
                ProjectApi::assign_tickets(
                    ctx.props().id,
                    tickets.iter().collect::<Vec<u64>>(),
                    callback,
                );
                self.app_state.close_dialog.emit(());
            }
            Msg::SubmittedForm() => {
                log::debug!("Form submitted");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self {
            project,
            user,
            ticket_list,
            app_state: _,
            _listener,
        } = self;

        let on_assign_click = |_| Msg::OpenSelectDialog();
        let on_add_click = |_| Msg::OpenFormDialog();

        let tickets = ticket_list.iter().map(
            |TicketDto {
                 id,
                 title,
                 description: _,
                 project_id: _,
                 status,
                 user_id: _,
             }| {
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
                    None => html! { <></> },
                }
            },
        );

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
                                    {
                                        if self.ticket_list.is_empty() {
                                            html! { <em>{ "No associated tickets" }</em> }
                                        } else {
                                            html! {
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
                                            }
                                        }
                                    }
                                    <div class="field is-grouped mt-6">
                                        <p class="control">
                                            <button class="button" onclick={ctx.link().callback(on_assign_click)}>
                                                <span class="icon is-small">
                                                    <i class="fas fa-arrow-up"></i>
                                                </span>
                                                <span>{ "Assign a ticket" }</span>
                                            </button>
                                        </p>
                                        <p class="control">
                                            <button class="button" onclick={ctx.link().callback(on_add_click)}>
                                                <span class="icon is-small">
                                                    <i class="fas fa-plus"></i>
                                                </span>
                                                <span>{ "Create a new one" }</span>
                                            </button>
                                        </p>
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
