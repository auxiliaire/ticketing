use crate::app_state::AppStateContext;
use crate::components::button_link::{ButtonLink, ButtonLinkData};
use crate::components::priority_tag::PriorityTag;
use crate::helpers::event_helper::get_file_from_change_event;
use crate::route::Route;
use crate::services::project_service::ProjectService;
use crate::services::{ticket_service::TicketService, user_service::UserService};
use implicit_clone::sync::IString;
use shared::dtos::identity::Identity;
use shared::dtos::project_dto::ProjectDto;
use shared::dtos::ticket_dto::TicketDto;
use shared::dtos::user_dto::UserDto;
use std::rc::Rc;
use yew::prelude::*;

const BUTTON_CLASS: &str = "button is-one-third";
const UPLOAD_ICON_CLASS: &str = "fas";
const FILE_INPUT_CLASS: &str = "file mb-2";

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

pub enum Msg {
    ContextChanged(AppStateContext),
    FetchedTicket(TicketDto),
    FetchedProject(ProjectDto),
    FetchedUser(UserDto),
    Subscribe,
    Subscribed(bool),
    Upload(Event),
    Uploaded(bool),
}

pub struct TicketPage {
    ticket: TicketDto,
    project: Option<ButtonLinkData<Route>>,
    user: Option<ButtonLinkData<Route>>,
    is_subscribed: bool,
    is_loading: bool,
    is_uploaded: bool,
    file_name: IString,
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}

impl Component for TicketPage {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(Msg::ContextChanged))
            .expect("context to be set");
        if let Some(Identity { token, .. }) = &app_state.identity {
            TicketService::fetch(
                token.to_string(),
                ctx.props().id,
                ctx.link().callback(Msg::FetchedTicket),
            );
            TicketService::is_subscribed(
                token.to_string(),
                ctx.props().id,
                ctx.link().callback(Msg::Subscribed),
            );
        }
        Self {
            ticket: TicketDto::default(),
            project: None,
            user: None,
            is_subscribed: false,
            is_loading: false,
            is_uploaded: false,
            file_name: IString::default(),
            app_state,
            _listener,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if let Some(Identity { token, .. }) = &self.app_state.identity {
            TicketService::fetch(
                token.to_string(),
                ctx.props().id,
                ctx.link().callback(Msg::FetchedTicket),
            );
        }
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContextChanged(state) => {
                self.app_state = state;
            }
            Msg::FetchedTicket(ticket) => {
                self.ticket = ticket;
                if let Some(Identity { token, .. }) = &self.app_state.identity {
                    if let Some(project_id) = self.ticket.project_id {
                        ProjectService::fetch(
                            token.to_string(),
                            project_id,
                            ctx.link().callback(Msg::FetchedProject),
                        );
                    }
                    if let Some(user_id) = self.ticket.user_id {
                        UserService::fetch(
                            token.to_string(),
                            user_id,
                            ctx.link().callback(Msg::FetchedUser),
                        );
                    }
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
                        id: user.public_id.unwrap(),
                    },
                });
            }
            Msg::Subscribe => {
                if let Some(Identity { token, .. }) = &self.app_state.identity {
                    TicketService::subscribe(
                        token.to_string(),
                        ctx.props().id,
                        ctx.link().callback(Msg::Subscribed),
                    );
                }
            }
            Msg::Subscribed(res) => {
                self.is_subscribed = res;
            }
            Msg::Upload(e) => {
                if let (Some(Identity { token, .. }), Some(file)) =
                    (&self.app_state.identity, get_file_from_change_event(e))
                {
                    self.is_loading = true;
                    self.file_name = IString::from(file.name());
                    TicketService::upload_attachment(
                        token.to_string(),
                        ctx.props().id,
                        file,
                        ctx.link().callback(Msg::Uploaded),
                    );
                }
            }
            Msg::Uploaded(res) => {
                self.is_loading = false;
                self.is_uploaded = res;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self {
            ticket,
            project,
            user,
            is_subscribed: has_subscribed,
            ..
        } = self;
        let priority = Rc::new(ticket.priority.clone());

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
                                        <div class="column is-one-quarter"><h5 class="title is-5">{ "Priority" }</h5></div>
                                        <div class="column"><PriorityTag {priority} /></div>
                                    </div>
                                    <div class="columns">
                                        <div class="column is-one-quarter"><h5 class="title is-5">{ "Status" }</h5></div>
                                        <div class="column"><span class="tag is-white">{ &ticket.status.to_string() }</span></div>
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
                    <div class="tile">
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-light">
                                <div class="content">
                                    <p class="title">{ "Actions" }</p>
                                    <p class="buttons">
                                        <button class={classes!(self.get_subscribe_button_classes())} onclick={ctx.link().callback(|_| Msg::Subscribe)}>
                                            <span class="icon is-small"><i class="fa-solid fa-satellite-dish"></i></span>
                                            <span>
                                                {
                                                    if *has_subscribed {
                                                        html! { "Unsubscribe" }
                                                    } else {
                                                        html! { "Subscribe" }
                                                    }
                                                }
                                            </span>
                                        </button>
                                        <div class={classes!(self.get_file_input_classes())}>
                                            <label class="file-label">
                                                <input class="file-input" type="file" name="file" onchange={ctx.link().callback(Msg::Upload)} />
                                                <span class="file-cta">
                                                    <span class="file-icon">
                                                        <i class={classes!(self.get_upload_icon_classes())}></i>
                                                    </span>
                                                    <span class="file-label">
                                                        { "Upload a file…" }
                                                    </span>
                                                </span>
                                                {
                                                    if !self.file_name.is_empty() {
                                                        html! {
                                                            <span class="file-name">
                                                                { self.file_name.to_string() }
                                                            </span>
                                                        }
                                                    } else {
                                                        html! { <></> }
                                                    }
                                                }
                                            </label>
                                        </div>
                                    </p>
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl TicketPage {
    fn get_subscribe_button_classes(&self) -> String {
        let mut classes = vec![BUTTON_CLASS];
        if self.is_subscribed {
            classes.push("is-success");
        }
        classes.join(" ")
    }

    fn get_upload_icon_classes(&self) -> String {
        let mut classes = vec![UPLOAD_ICON_CLASS];
        if self.is_loading {
            classes.push("fa-spinner");
            classes.push("fa-pulse");
        } else {
            classes.push("fa-upload");
        }
        classes.join(" ")
    }

    fn get_file_input_classes(&self) -> String {
        let mut classes = vec![FILE_INPUT_CLASS];
        if self.is_subscribed {
            classes.push("is-success");
        }
        if !self.file_name.is_empty() {
            classes.push("has-name")
        }
        classes.join(" ")
    }
}
