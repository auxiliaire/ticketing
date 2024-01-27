use crate::app_state::AppStateContext;
use crate::components::button_link::{ButtonLink, ButtonLinkData};
use crate::components::check_tag::CheckTag;
use crate::components::dialogs::form_dialog::FormDialog;
use crate::components::dialogs::select_dialog::SelectDialog;
use crate::components::event_helper::{get_transfer_data, set_transfer_data};
use crate::components::forms::ticket_form::TicketForm;
use crate::services::project_service::ProjectService;
use crate::services::ticket_service::TicketService;
use crate::services::user_service::UserService;
use crate::{app_state::AppState, dialog::Dialog, route::Route};
use implicit_clone::sync::{IArray, IString};
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::identity::Identity;
use shared::dtos::project_dto::ProjectDto;
use shared::dtos::ticket_dto::TicketDto;
use shared::dtos::user_dto::UserDto;
use shared::validation::ticket_validation::TicketStatus;
use std::rc::Rc;
use strum::IntoEnumIterator;
use web_sys::DragEvent;
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

pub enum ProjectBoardPageMsg {
    ContextChanged(AppStateContext),
    FetchedProject(ProjectDto),
    FetchedUser(UserDto),
    FetchedTickets(Vec<TicketDto>),
    FetchUnassigned(Callback<Vec<TicketDto>>),
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
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}

impl Component for ProjectBoardPage {
    type Message = ProjectBoardPageMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(ProjectBoardPageMsg::ContextChanged))
            .expect("context to be set");

        ProjectBoardPage::init(&app_state, ctx);
        Self {
            project: ProjectDto::default(),
            user: None,
            ticket_list: vec![],
            app_state,
            _listener,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if let Some(Identity { token, .. }) = &self.app_state.identity {
            ProjectService::fetch(
                token.clone(),
                ctx.props().id,
                ctx.link().callback(ProjectBoardPageMsg::FetchedProject),
            );
        }
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProjectBoardPageMsg::FetchedProject(project) => {
                self.project = project;
                if let Some(Identity { token, .. }) = &self.app_state.identity {
                    UserService::fetch(
                        token.clone(),
                        self.project.user_id,
                        ctx.link().callback(ProjectBoardPageMsg::FetchedUser),
                    );
                }
            }
            ProjectBoardPageMsg::FetchedUser(user) => {
                self.user = Some(ButtonLinkData {
                    label: IString::from(user.name),
                    to: Route::User {
                        id: user.public_id.unwrap(),
                    },
                });
            }
            ProjectBoardPageMsg::FetchedTickets(tickets) => {
                self.ticket_list = tickets;
            }
            ProjectBoardPageMsg::ContextChanged(state) => {
                self.app_state = state;
                ProjectBoardPage::init(&self.app_state, ctx);
            }
            ProjectBoardPageMsg::OpenSelectDialog() => {
                let optionsapi: Callback<Callback<Vec<TicketDto>>> =
                    ctx.link().callback(ProjectBoardPageMsg::FetchUnassigned);
                let onselect: Callback<IArray<u64>> =
                    ctx.link().callback(ProjectBoardPageMsg::SelectedTickets);
                let dialog = Rc::new(Dialog {
                    active: true,
                    content: html! {
                        <SelectDialog<u64, TicketDto> {optionsapi} {onselect} />
                    },
                });
                AppState::update_dialog(&self.app_state, dialog);
            }
            ProjectBoardPageMsg::OpenFormDialog() => {
                let dialog = Rc::new(Dialog {
                    active: true,
                    content: html! {
                        <FormDialog title="Create a new Ticket">
                            <TicketForm projectid={ctx.props().id} onsubmit={ctx.link().callback(ProjectBoardPageMsg::SubmittedForm)} />
                        </FormDialog>
                    },
                });
                AppState::update_dialog(&self.app_state, dialog);
            }
            ProjectBoardPageMsg::OpenTicketDialog(ticketid) => {
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
                                <TicketForm {ticketid} projectid={ctx.props().id} onsubmit={ctx.link().callback(ProjectBoardPageMsg::SubmittedForm)} />
                            </FormDialog>
                        },
                    });
                    AppState::update_dialog(&self.app_state, dialog);
                }
            }
            ProjectBoardPageMsg::SelectedTickets(tickets) => {
                if let Some(Identity { token, .. }) = &self.app_state.identity {
                    let callback = ctx.link().callback(ProjectBoardPageMsg::FetchedTickets);
                    ProjectService::assign_tickets(
                        token.to_string(),
                        ctx.props().id,
                        tickets.iter().collect::<Vec<u64>>(),
                        callback,
                    );
                }
                AppState::close_dialog(&self.app_state);
            }
            ProjectBoardPageMsg::SubmittedForm((ticket, callback_error)) => {
                log::debug!("Form submitted: {}", ticket);
                if let Some(Identity { token, .. }) = &self.app_state.identity {
                    if ticket.id.is_some() {
                        TicketService::update(
                            token.to_string(),
                            ticket,
                            ctx.link().callback(ProjectBoardPageMsg::TicketCreated),
                            callback_error,
                        );
                    } else {
                        TicketService::create(
                            token.to_string(),
                            ticket,
                            ctx.link().callback(ProjectBoardPageMsg::TicketCreated),
                            callback_error,
                        );
                    }
                }
            }
            ProjectBoardPageMsg::TicketCreated(ticket) => {
                log::debug!("Created: {}", ticket);
                AppState::close_dialog(&self.app_state);
                if let Some(Identity { token, .. }) = &self.app_state.identity {
                    ProjectService::fetch_assigned_tickets(
                        token.clone(),
                        ctx.props().id,
                        ctx.link().callback(ProjectBoardPageMsg::FetchedTickets),
                    );
                }
            }
            ProjectBoardPageMsg::TicketUpdated(ticket) => {
                log::debug!("Updated: {}", ticket);
            }
            ProjectBoardPageMsg::DragStart(e, id) => {
                log::debug!("Drag started. Id: {}", id);
                let _ = set_transfer_data(e, format!("{}", id).as_str());
            }
            ProjectBoardPageMsg::Drop(e, status) => {
                e.prevent_default();
                if let Ok(id_s) = get_transfer_data(e) {
                    if let Ok(id) = id_s.as_str().parse::<u64>() {
                        log::debug!("Dropped. Status: id({}) -> {}", id, status);
                        if let Some(Identity { token, .. }) = &self.app_state.identity {
                            if let Some(ticket) = self
                                .ticket_list
                                .iter()
                                .filter(|t| t.id == Some(id))
                                .collect::<Vec<&TicketDto>>()
                                .first()
                            {
                                if ticket.status != status {
                                    TicketService::update(
                                        token.to_string(),
                                        TicketDto {
                                            id: Some(id),
                                            title: ticket.title.clone(),
                                            status,
                                            description: ticket.description.clone(),
                                            project_id: ticket.project_id,
                                            user_id: ticket.user_id,
                                            priority: ticket.priority.clone(),
                                        },
                                        ctx.link().callback(ProjectBoardPageMsg::TicketCreated),
                                        Callback::noop(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
            ProjectBoardPageMsg::FetchUnassigned(consumer) => {
                if let Some(Identity { token, .. }) = &self.app_state.identity {
                    TicketService::fetch_unassigned(token.to_string(), consumer);
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

        let on_assign_click = |_| ProjectBoardPageMsg::OpenSelectDialog();
        let on_add_click = |_| ProjectBoardPageMsg::OpenFormDialog();

        let status_headers = TicketStatus::iter()
            .filter(Self::is_status_managable)
            .map(|status| html! {
                <div class="tile col"><div class="tile notification is-light is-vertical"><p class="tile is-uppercase is-size-7"><b>{ html! {status}}</b></p></div></div>
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
                                        <div class="field has-addons">
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
                                        <div class="field ml-3">
                                            <p class="control">
                                                <Link<Route> classes={classes!("button")} to={Route::Project { id: project.id.unwrap_or(0) }}>
                                                    <span class="icon">
                                                        <i class="fa-solid fa-list-ul"></i>
                                                    </span>
                                                    <span>{ "List view" }</span>
                                                </Link<Route>>
                                            </p>
                                        </div>
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
    fn init(app_state: &AppStateContext, ctx: &Context<Self>) {
        if let Some(Identity { token, .. }) = &app_state.identity {
            ProjectService::fetch(
                token.to_string(),
                ctx.props().id,
                ctx.link().callback(ProjectBoardPageMsg::FetchedProject),
            );
            ProjectService::fetch_assigned_tickets(
                token.to_string(),
                ctx.props().id,
                ctx.link().callback(ProjectBoardPageMsg::FetchedTickets),
            );
        }
    }

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
        let function = move |e: DragEvent| ProjectBoardPageMsg::Drop(e, column);
        ctx.link().callback(function)
    }

    fn dragstart_callback(ctx: &Context<Self>, id: u64) -> Callback<DragEvent> {
        let function = move |e: DragEvent| ProjectBoardPageMsg::DragStart(e, id);
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
            "grabable",
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
        let function = move |_: MouseEvent| ProjectBoardPageMsg::OpenTicketDialog(id);
        ctx.link().callback(function)
    }
}
