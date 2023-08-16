use crate::components::bulma::tables::data_sources::ticket_data_source::TicketDataSource;
use crate::components::bulma::tables::table::Table;
use crate::components::bulma::tables::table_data_source::ITableDataSource;
use crate::components::bulma::tables::table_head_data::TableHeadData;
use crate::components::button_link::{ButtonLink, ButtonLinkData};
use crate::components::check_tag::CheckTag;
use crate::components::dialogs::form_dialog::FormDialog;
use crate::components::dialogs::select_dialog::SelectDialog;
use crate::components::forms::ticket_form::TicketForm;
use crate::components::option_data::OptionData;
use crate::services::project_service::ProjectService;
use crate::services::user_service::UserService;
use crate::{AppState, Dialog, Route};
use frontend::services::ticket_service::TicketService;
use implicit_clone::{
    sync::{IArray, IString},
    unsync,
};
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::project_dto::ProjectDto;
use shared::dtos::ticket_dto::{ITicketDto, TicketDto, TicketField, TicketValue};
use shared::dtos::user_dto::UserDto;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::Link;

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

pub enum ProjectPageMsg {
    FetchedProject(ProjectDto),
    FetchedUser(UserDto),
    FetchedTickets(Vec<TicketDto>),
    ContextChanged(Rc<AppState>),
    OpenSelectDialog(),
    OpenFormDialog(),
    SelectedTickets(IArray<u64>),
    SubmittedForm((TicketDto, Callback<ErrorResponse>)),
    SortTickets(TableHeadData),
    TicketCreated(TicketDto),
}

pub struct ProjectPage {
    project: ProjectDto,
    user: Option<ButtonLinkData<Route>>,
    ticket_list: Vec<TicketDto>,
    app_state: Rc<AppState>,
    _listener: ContextHandle<Rc<AppState>>,
}
impl Component for ProjectPage {
    type Message = ProjectPageMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ProjectService::fetch(
            ctx.props().id,
            ctx.link().callback(ProjectPageMsg::FetchedProject),
        );
        ProjectService::fetch_assigned_tickets(
            ctx.props().id,
            ctx.link().callback(ProjectPageMsg::FetchedTickets),
        );
        let (app_state, _listener) = ctx
            .link()
            .context::<Rc<AppState>>(ctx.link().callback(ProjectPageMsg::ContextChanged))
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
        ProjectService::fetch(
            ctx.props().id,
            ctx.link().callback(ProjectPageMsg::FetchedProject),
        );
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProjectPageMsg::FetchedProject(project) => {
                self.project = project;
                UserService::fetch(
                    self.project.user_id,
                    ctx.link().callback(ProjectPageMsg::FetchedUser),
                );
            }
            ProjectPageMsg::FetchedUser(user) => {
                self.user = Some(ButtonLinkData {
                    label: IString::from(user.name),
                    to: Route::User {
                        id: user.id.unwrap(),
                    },
                });
            }
            ProjectPageMsg::FetchedTickets(tickets) => {
                self.ticket_list = tickets;
            }
            ProjectPageMsg::ContextChanged(state) => {
                self.app_state = state;
            }
            ProjectPageMsg::OpenSelectDialog() => {
                let optionsapi: Callback<Callback<Vec<TicketDto>>> =
                    Callback::from(TicketService::fetch_unassigned);
                let onselect: Callback<IArray<u64>> =
                    ctx.link().callback(ProjectPageMsg::SelectedTickets);
                let dialog = Rc::new(Dialog {
                    active: true,
                    content: html! {
                        <SelectDialog<u64, TicketDto> {optionsapi} {onselect} />
                    },
                });
                self.app_state.update_dialog.emit(dialog);
            }
            ProjectPageMsg::OpenFormDialog() => {
                let dialog = Rc::new(Dialog {
                    active: true,
                    content: html! {
                        <FormDialog title="Create a new Ticket">
                            <TicketForm projectid={ctx.props().id} onsubmit={ctx.link().callback(ProjectPageMsg::SubmittedForm)} />
                        </FormDialog>
                    },
                });
                self.app_state.update_dialog.emit(dialog);
            }
            ProjectPageMsg::SelectedTickets(tickets) => {
                let callback = ctx.link().callback(ProjectPageMsg::FetchedTickets);
                ProjectService::assign_tickets(
                    ctx.props().id,
                    tickets.iter().collect::<Vec<u64>>(),
                    callback,
                );
                self.app_state.close_dialog.emit(());
            }
            ProjectPageMsg::SubmittedForm((ticket, callback_error)) => {
                log::debug!("Form submitted: {}", ticket);
                TicketService::create(
                    ticket,
                    ctx.link().callback(ProjectPageMsg::TicketCreated),
                    callback_error,
                );
            }
            ProjectPageMsg::SortTickets(sortdata) => TicketService::fetch_all(
                Some(ctx.props().id),
                None,
                sortdata.sort.as_ref().map(|s| s.sort.clone()),
                sortdata
                    .sort
                    .as_ref()
                    .map(|s| unsync::IString::from(s.order.to_string())),
                ctx.link().callback(ProjectPageMsg::FetchedTickets),
            ),
            ProjectPageMsg::TicketCreated(ticket) => {
                log::debug!("Created: {}", ticket);
                self.app_state.close_dialog.emit(());
                ProjectService::fetch_assigned_tickets(
                    ctx.props().id,
                    ctx.link().callback(ProjectPageMsg::FetchedTickets),
                );
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

        let on_assign_click = |_| ProjectPageMsg::OpenSelectDialog();
        let on_add_click = |_| ProjectPageMsg::OpenFormDialog();

        let datasource: ITableDataSource<TicketField, ITicketDto, TicketValue> =
            TicketDataSource::from(ticket_list).into();

        let sorthandler = Some(ctx.link().callback(ProjectPageMsg::SortTickets));

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
                                                <Link<Route> classes={classes!("button")} to={Route::ProjectBoard { id: project.id.unwrap_or(0) }}>
                                                    <span class="icon">
                                                        <i class="fa-solid fa-table-columns"></i>
                                                    </span>
                                                    <span>{ "Board view" }</span>
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
                    <div class="tile">
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-light">
                                <div class="content">
                                    <p class="title">{ "Tickets" }</p>
                                    <Table<TicketField, ITicketDto, TicketValue> {datasource} {sorthandler} />
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
