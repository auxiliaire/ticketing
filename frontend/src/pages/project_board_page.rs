use crate::components::button_link::{ButtonLink, ButtonLinkData};
use crate::components::check_tag::CheckTag;
use crate::components::dialogs::form_dialog::FormDialog;
use crate::components::dialogs::select_dialog::SelectDialog;
use crate::components::event_helper::{get_transfer_data, set_transfer_data};
use crate::components::forms::ticket_form::TicketForm;
use crate::services::project_service::ProjectService;
use crate::services::user_service::UserService;
use crate::{AppState, Dialog, Route};
use frontend::services::ticket_service::TicketService;
use implicit_clone::sync::{IArray, IString};
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::project_dto::ProjectDto;
use shared::dtos::ticket_dto::TicketDto;
use shared::dtos::user_dto::UserDto;
use shared::validation::ticket_validation::TicketStatus;
use std::rc::Rc;
use strum::IntoEnumIterator;
use web_sys::DragEvent;
use yew::prelude::*;

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
    OpenTicketDialog(u64),
    SelectedTickets(IArray<u64>),
    SubmittedForm((TicketDto, Callback<ErrorResponse>)),
    TicketCreated(TicketDto),
    TicketUpdated(TicketDto),
    DragStart(DragEvent, u64),
    Drop(DragEvent, TicketStatus),
}

pub struct ProjectBoardPage {
    project: ProjectDto,
    user: Option<ButtonLinkData<Route>>,
    ticket_list: Vec<TicketDto>,
    app_state: Rc<AppState>,
    _listener: ContextHandle<Rc<AppState>>,
}
impl Component for ProjectBoardPage {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ProjectService::fetch(ctx.props().id, ctx.link().callback(Msg::FetchedProject));
        ProjectService::fetch_assigned_tickets(
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
        ProjectService::fetch(ctx.props().id, ctx.link().callback(Msg::FetchedProject));
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchedProject(project) => {
                self.project = project;
                UserService::fetch(self.project.user_id, ctx.link().callback(Msg::FetchedUser));
            }
            Msg::FetchedUser(user) => {
                self.user = Some(ButtonLinkData {
                    label: IString::from(user.name),
                    to: Route::User {
                        id: user.id.unwrap(),
                    },
                });
            }
            Msg::FetchedTickets(tickets) => {
                self.ticket_list = tickets;
            }
            Msg::ContextChanged(state) => {
                self.app_state = state;
            }
            Msg::OpenSelectDialog() => {
                let optionsapi: Callback<Callback<Vec<TicketDto>>> =
                    Callback::from(TicketService::fetch_unassigned);
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
                let dialog = Rc::new(Dialog {
                    active: true,
                    content: html! {
                        <FormDialog title="Create a new Ticket">
                            <TicketForm projectid={ctx.props().id} onsubmit={ctx.link().callback(Msg::SubmittedForm)} />
                        </FormDialog>
                    },
                });
                self.app_state.update_dialog.emit(dialog);
            }
            Msg::OpenTicketDialog(ticketid) => {
                if let Some(ticket) = self
                    .ticket_list
                    .iter()
                    .filter(|t| t.id == Some(ticketid))
                    .collect::<Vec<&TicketDto>>()
                    .first()
                {
                    let dialog = Rc::new(Dialog {
                        active: true,
                        content: html! {
                            <FormDialog title={ticket.title.clone()}>
                                <TicketForm {ticketid} projectid={ctx.props().id} onsubmit={ctx.link().callback(Msg::SubmittedForm)} />
                            </FormDialog>
                        },
                    });
                    self.app_state.update_dialog.emit(dialog);
                }
            }
            Msg::SelectedTickets(tickets) => {
                let callback = ctx.link().callback(Msg::FetchedTickets);
                ProjectService::assign_tickets(
                    ctx.props().id,
                    tickets.iter().collect::<Vec<u64>>(),
                    callback,
                );
                self.app_state.close_dialog.emit(());
            }
            Msg::SubmittedForm((ticket, callback_error)) => {
                log::debug!("Form submitted: {}", ticket);
                if ticket.id.is_some() {
                    TicketService::update(
                        ticket,
                        ctx.link().callback(Msg::TicketCreated),
                        callback_error,
                    );
                } else {
                    TicketService::create(
                        ticket,
                        ctx.link().callback(Msg::TicketCreated),
                        callback_error,
                    );
                }
            }
            Msg::TicketCreated(ticket) => {
                log::debug!("Created: {}", ticket);
                self.app_state.close_dialog.emit(());
                ProjectService::fetch_assigned_tickets(
                    ctx.props().id,
                    ctx.link().callback(Msg::FetchedTickets),
                );
            }
            Msg::TicketUpdated(ticket) => {
                log::debug!("Updated: {}", ticket);
            }
            Msg::DragStart(e, id) => {
                log::debug!("Drag started. Id: {}", id);
                let _ = set_transfer_data(e, format!("{}", id).as_str());
            }
            Msg::Drop(e, status) => {
                e.prevent_default();
                if let Ok(id_s) = get_transfer_data(e) {
                    if let Ok(id) = id_s.as_str().parse::<u64>() {
                        log::debug!("Dropped. Status: id({}) -> {}", id, status);
                        if let Some(ticket) = self
                            .ticket_list
                            .iter()
                            .filter(|t| t.id == Some(id))
                            .collect::<Vec<&TicketDto>>()
                            .first()
                        {
                            if ticket.status != status {
                                TicketService::update(
                                    TicketDto {
                                        id: Some(id),
                                        title: ticket.title.clone(),
                                        status,
                                        description: ticket.description.clone(),
                                        project_id: ticket.project_id,
                                        user_id: ticket.user_id,
                                        priority: ticket.priority.clone(),
                                    },
                                    ctx.link().callback(Msg::TicketCreated),
                                    Callback::noop(),
                                );
                            }
                        }
                    }
                }
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

        let status_headers = TicketStatus::iter()
            .filter(Self::is_status_managable)
            .map(|status| html! {
                <div class="tile col"><div class="tile notification is-light is-vertical"><p class="tile is-uppercase is-size-7"><b>{ status }</b></p></div></div>
            });

        let statuses = TicketStatus::iter()
            .filter(Self::is_status_managable)
            .map(
            |status| {
                let ondrop = Self::drop_callback(ctx, status);
                html! {
                    <div class="tile col">
                        <div class="tile notification is-light is-vertical pt-3 pr-3 pb-3 pl-3" {ondrop} ondragover={|e: DragEvent| e.prevent_default()}>
                            { Self::ticket_view(ctx, &status, ticket_list) }
                        </div>
                    </div>
                }
            }
        );

        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-light">
                            <div class="columns">
                                <div class="column is-two-thirds">
                                    <p class="title">{ &project.summary }</p>
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
                                <div class="column">
                                    <div class="content">
                                        <p class="title is-5">{ "Details" }</p>
                                        <div class="columns mb-0">
                                            <div class="column"><h6 class="title is-6">{ "Deadline" }</h6></div>
                                            <div class="column">{ &project.deadline.map_or(String::from("-"), |d| d.format("%F").to_string()) }</div>
                                        </div>
                                        <div class="columns mb-0">
                                            <div class="column"><h6 class="title is-6">{ "Created by" }</h6></div>
                                            <div class="column">
                                                <ButtonLink<Route> data={user.clone()} />
                                            </div>
                                        </div>
                                        <div class="columns mb-0">
                                            <div class="column"><h6 class="title is-6">{ "Active" }</h6></div>
                                            <div class="column">
                                                <CheckTag checked={project.active == 1} />
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </article>
                    </div>
                    <div class="tile is-parent">
                        { for status_headers }
                    </div>
                    <div class="tile is-parent">
                        { for statuses }
                    </div>
                </div>
            </div>
        }
    }
}

impl ProjectBoardPage {
    fn is_status_managable(status: &TicketStatus) -> bool {
        status != &TicketStatus::Created && status != &TicketStatus::Closed
    }

    fn ticket_view(ctx: &Context<Self>, column: &TicketStatus, ticket_list: &[TicketDto]) -> Html {
        let tickets = ticket_list.iter().map(
            |TicketDto {
                 id,
                 title,
                 description: _,
                 project_id: _,
                 status,
                 user_id: _,
                 priority: _,
             }| {
                match id {
                    Some(id) => {
                        let ondragstart = Self::dragstart_callback(ctx, *id);
                        let onclick = Self::ticket_click_callback(ctx, *id);
                        html! {
                            <div class={classes!(Self::ticket_classes(*status, *column))} draggable="true" {ondragstart}>
                                <a draggable="true" ondragstart={|e: DragEvent| e.prevent_default()} {onclick}>
                                    {title.clone()}
                                </a>
                            </div>
                        }
                    },
                    None => html! { <></> },
                }
            },
        );
        html! { for tickets }
    }

    fn drop_callback(ctx: &Context<Self>, column: TicketStatus) -> Callback<DragEvent> {
        let function = move |e: DragEvent| Msg::Drop(e, column);
        ctx.link().callback(function)
    }

    fn dragstart_callback(ctx: &Context<Self>, id: u64) -> Callback<DragEvent> {
        let function = move |e: DragEvent| Msg::DragStart(e, id);
        ctx.link().callback(function)
    }

    fn ticket_classes(status: TicketStatus, column: TicketStatus) -> Vec<&'static str> {
        let mut cls = vec![
            "tile",
            "notification",
            "ticket",
            "is-white",
            "pt-3",
            "pr-3",
            "pb-3",
            "pl-3",
            "is-clickable",
        ];
        if !Self::is_ticket_visible(&status, &column) {
            cls.push("is-invisible");
        }
        cls
    }

    fn is_ticket_visible(status: &TicketStatus, column: &TicketStatus) -> bool {
        status == column
    }

    fn ticket_click_callback(ctx: &Context<Self>, id: u64) -> Callback<MouseEvent> {
        let function = move |_: MouseEvent| Msg::OpenTicketDialog(id);
        ctx.link().callback(function)
    }
}
