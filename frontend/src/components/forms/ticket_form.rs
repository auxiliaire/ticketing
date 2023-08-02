use crate::components::bulma::field::Field;
use crate::components::dialogs::dialog_context::DialogContext;
use crate::components::html::select::Select;
use crate::components::html::text_input::TextInput;
use entity::sea_orm_active_enums::Priority;
use frontend::services::user_service::UserService;
use gloo_timers::callback::Timeout;
use implicit_clone::unsync::{IArray, IString};
use serde_valid::Validate;
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::ticket_dto::TicketDto;
use shared::dtos::user_dto::UserDto;
use shared::validation::is_empty::IsEmpty;
use shared::validation::ticket_validation::{TicketPriority, TicketStatus};
use shared::validation::validation_messages::{
    ErrorsWrapper, IValidationMessages, ValidationMessagesTrait,
};
use std::rc::Rc;
use std::str::FromStr;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

const SEARCH_DELAY_MS: u32 = 300;
const DROPDOWN_CLOSE_MS: u32 = 200;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub onsubmit: Callback<(TicketDto, Callback<ErrorResponse>)>,
    pub projectid: Option<u64>,
}

pub enum TicketMsg {
    DialogContextChanged(Rc<DialogContext>),
    UpdateTitle(AttrValue),
    UpdateDescription(AttrValue),
    UpdateProjectId(AttrValue),
    UpdatePriority(AttrValue),
    UpdateStatus(AttrValue),
    UpdateOwner(AttrValue),
    UpdateUserId((u64, IString)),
    SearchUser(AttrValue),
    ToggleSearchDropdownDelayed(bool),
    ToggleSearchDropdown(bool),
    FetchedUsers(Vec<UserDto>),
    Submit(),
    UpdateErrors(ErrorResponse),
    Cancel(),
}

pub struct TicketForm {
    dialog_context: Option<Rc<DialogContext>>,
    ticket: TicketDto,
    project_id: Option<u64>,
    owner: IString,
    user_search: IString,
    search_timeout: Option<Timeout>,
    dropdown_enabled: bool,
    user_list: IArray<(u64, IString)>,
    on_submit: Callback<(TicketDto, Callback<ErrorResponse>)>,
    common_error: IValidationMessages,
    title_error: IValidationMessages,
    description_error: IValidationMessages,
    project_error: IValidationMessages,
    owner_error: IValidationMessages,
    priority_error: IValidationMessages,
    status_error: IValidationMessages,
}
impl Component for TicketForm {
    type Message = TicketMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let option_dialog_context = ctx
            .link()
            .context::<Rc<DialogContext>>(ctx.link().callback(TicketMsg::DialogContextChanged));
        let dialog_context = option_dialog_context.map(|(context, _listener)| context);
        let status: TicketStatus = match ctx.props().projectid.is_some() {
            true => TicketStatus::Selected,
            false => TicketStatus::Created,
        };

        Self {
            dialog_context,
            ticket: TicketDto {
                project_id: ctx.props().projectid,
                status,
                ..Default::default()
            },
            project_id: ctx.props().projectid,
            owner: IString::from(""),
            user_search: IString::from(""),
            search_timeout: None,
            dropdown_enabled: false,
            user_list: IArray::from(vec![]),
            on_submit: ctx.props().onsubmit.to_owned(),
            common_error: None,
            title_error: None,
            description_error: None,
            project_error: None,
            owner_error: None,
            priority_error: None,
            status_error: None,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TicketMsg::DialogContextChanged(context) => {
                self.dialog_context = Some(context);
            }
            TicketMsg::UpdateTitle(title) => {
                self.ticket.title = String::from(title.as_str());
            }
            TicketMsg::UpdateDescription(description) => {
                self.ticket.description = String::from(description.as_str());
            }
            TicketMsg::UpdateProjectId(value) => {
                self.ticket.project_id = value.as_str().parse::<u64>().ok();
            }
            TicketMsg::UpdatePriority(value) => {
                if let Ok(priority) = TicketPriority::try_from(value.as_str()) {
                    self.ticket.priority = priority;
                }
            }
            TicketMsg::UpdateStatus(value) => {
                if let Ok(status) = TicketStatus::from_str(value.as_str()) {
                    self.ticket.status = status;
                }
            }
            TicketMsg::UpdateOwner(value) => {
                self.owner = value;
                self.ticket.user_id = self.owner.as_str().parse::<u64>().ok();
            }
            TicketMsg::UpdateUserId((id, name)) => {
                self.owner = IString::from(format!("{}", id));
                self.ticket.user_id = Some(id);
                self.user_search = name;
            }
            TicketMsg::SearchUser(value) => {
                self.user_search = value;
                // We need to throttle the API call to prevent superfluous calls
                let q = self.user_search.clone();
                let fetch_callback = ctx.link().callback(TicketMsg::FetchedUsers);
                if let Some(timeout) = self.search_timeout.take() {
                    timeout.cancel();
                }
                self.search_timeout = Some(Timeout::new(SEARCH_DELAY_MS, || {
                    UserService::fetch_all(Some(q), fetch_callback)
                }));
            }
            TicketMsg::ToggleSearchDropdownDelayed(value) => {
                let toggle_search_dropdown = ctx.link().callback(TicketMsg::ToggleSearchDropdown);
                let timeout = match value {
                    true => {
                        Timeout::new(DROPDOWN_CLOSE_MS, move || toggle_search_dropdown.emit(true))
                    }
                    false => Timeout::new(DROPDOWN_CLOSE_MS, move || {
                        toggle_search_dropdown.emit(false)
                    }),
                };
                timeout.forget();
            }
            TicketMsg::ToggleSearchDropdown(value) => {
                self.dropdown_enabled = value;
            }
            TicketMsg::FetchedUsers(list) => {
                let mut v = vec![];
                for u in list {
                    v.push((u.id.unwrap(), IString::from(u.name.clone())));
                }
                self.user_list = IArray::from(v);
            }
            TicketMsg::Submit() => {
                let result = self.ticket.validate();
                match result {
                    Ok(_) => self.on_submit.emit((
                        self.ticket.clone(),
                        ctx.link().callback(TicketMsg::UpdateErrors),
                    )),
                    Err(e) => self.update_errors(ErrorsWrapper(e)),
                }
            }
            TicketMsg::UpdateErrors(error_response) => {
                log::debug!("Error response: {}", error_response);
                if let Some(errors) = error_response.details {
                    self.update_errors(errors);
                }
            }
            TicketMsg::Cancel() => match self.dialog_context.clone() {
                Some(context) => {
                    context.closehandler.emit(());
                }
                None => {
                    let navigator = ctx.link().navigator().unwrap();
                    navigator.back();
                }
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.dialog_context.is_some() {
            true => html! {
                <>
                    <section class="modal-card-body">
                        {
                            match self.ticket.id.is_some() {
                                true => self.soft_body(ctx),
                                false => self.hard_body(ctx),
                            }
                        }
                    </section>
                    <footer class="modal-card-foot">
                        { self.submit_button(ctx) }
                        { self.cancel_button(ctx) }
                    </footer>
                </>
            },
            false => html! {
                <div class="card">
                    <div class="card-content">
                        { self.hard_body(ctx) }
                    </div>
                    <footer class="card-footer">
                        <div class="card-content">
                            <div class="field is-grouped">
                                <div class="control">
                                    { self.submit_button(ctx) }
                                </div>
                                <div class="control">
                                    { self.cancel_button(ctx) }
                                </div>
                            </div>
                        </div>
                    </footer>
                </div>
            },
        }
    }
}

impl TicketForm {
    fn soft_body(&self, ctx: &Context<Self>) -> Html {
        let users = self.user_list.iter().map(|t| {
            let select_user = ctx.link().callback(TicketMsg::UpdateUserId);
            let name = t.1.clone();
            html! {
                <a class="dropdown-item" onclick={move |_| {
                    select_user.emit((t.0, t.1.clone()));
                }}>{name}</a>
            }
        });

        let on_search_focus = ctx.link().callback(TicketMsg::ToggleSearchDropdown);
        let on_search_blur = ctx.link().callback(TicketMsg::ToggleSearchDropdownDelayed);

        html! {
            <div class="content">
                if let Some(common_error) = &self.common_error {
                    <p class="help is-danger">
                        <ul>
                        {
                            common_error.iter().map(|message| {
                                html!{<li>{ message }</li>}
                            }).collect::<Html>()
                        }
                        </ul>
                    </p>
                }
                <div class="columns">
                    <div class="column is-one-quarter"><h6 class="title is-6">{ "Title" }</h6></div>
                    <div class="column"><span>{ self.ticket.title.clone() }</span><TextInput base_classes={classes!("is-hidden")} value={self.ticket.title.clone()} on_change={ctx.link().callback(TicketMsg::UpdateTitle)} valid={self.title_error.is_empty()} /></div>
                </div>
                <Field label="Description" help={&self.description_error}>
                    <TextInput value={self.ticket.description.clone()} on_change={ctx.link().callback(TicketMsg::UpdateDescription)} valid={self.description_error.is_empty()} />
                </Field>
                <Field label="Project" help={&self.project_error}>
                    <TextInput value={self.get_project_id()} on_change={ctx.link().callback(TicketMsg::UpdateProjectId)} valid={self.project_error.is_empty()} />
                </Field>
                <Field label="Priority" help={&self.status_error}>
                    <Select value={self.ticket.priority.to_string()} options={self.get_priorities()} on_change={ctx.link().callback(TicketMsg::UpdatePriority)} valid={self.priority_error.is_empty()} />
                </Field>
                <Field label="Status" help={&self.status_error}>
                    <Select value={self.ticket.status.to_string()} options={self.get_statuses()} on_change={ctx.link().callback(TicketMsg::UpdateStatus)} valid={self.status_error.is_empty()} />
                </Field>
                <Field label="Owner" help={&self.owner_error}>
                    <TextInput value={self.owner.clone()} on_change={ctx.link().callback(TicketMsg::UpdateOwner)} valid={self.owner_error.is_empty()} base_classes="input is-hidden" />
                    <div class={classes!(self.get_dropdown_classes())}>
                        <div class="dropdown-trigger">
                            <TextInput value={self.user_search.clone()} on_change={ctx.link().callback(TicketMsg::SearchUser)} valid={self.owner_error.is_empty()}
                                on_focus={move |_| on_search_focus.emit(true)} on_blur={move |_| on_search_blur.emit(false)} />
                        </div>
                        <div class="dropdown-menu" role="menu">
                            <div class="dropdown-content">
                                { for users }
                            </div>
                        </div>
                    </div>
                </Field>
            </div>
        }
    }

    fn hard_body(&self, ctx: &Context<Self>) -> Html {
        let users = self.user_list.iter().map(|t| {
            let select_user = ctx.link().callback(TicketMsg::UpdateUserId);
            let name = t.1.clone();
            html! {
                <a class="dropdown-item" onclick={move |_| {
                    select_user.emit((t.0, t.1.clone()));
                }}>{name}</a>
            }
        });

        let on_search_focus = ctx.link().callback(TicketMsg::ToggleSearchDropdown);
        let on_search_blur = ctx.link().callback(TicketMsg::ToggleSearchDropdownDelayed);

        html! {
            <>
                if let Some(common_error) = &self.common_error {
                    <p class="help is-danger">
                        <ul>
                        {
                            common_error.iter().map(|message| {
                                html!{<li>{ message }</li>}
                            }).collect::<Html>()
                        }
                        </ul>
                    </p>
                }
                <Field label="Title" help={&self.title_error}>
                    <TextInput value={self.ticket.title.clone()} on_change={ctx.link().callback(TicketMsg::UpdateTitle)} valid={self.title_error.is_empty()} />
                </Field>
                <Field label="Description" help={&self.description_error}>
                    <TextInput value={self.ticket.description.clone()} on_change={ctx.link().callback(TicketMsg::UpdateDescription)} valid={self.description_error.is_empty()} />
                </Field>
                <Field label="Project" help={&self.project_error}>
                    <TextInput value={self.get_project_id()} on_change={ctx.link().callback(TicketMsg::UpdateProjectId)} valid={self.project_error.is_empty()} />
                </Field>
                <Field label="Priority" help={&self.status_error}>
                    <Select value={self.ticket.priority.to_string()} options={self.get_priorities()} on_change={ctx.link().callback(TicketMsg::UpdatePriority)} valid={self.priority_error.is_empty()} />
                </Field>
                <Field label="Status" help={&self.status_error}>
                    <Select value={self.ticket.status.to_string()} options={self.get_statuses()} on_change={ctx.link().callback(TicketMsg::UpdateStatus)} valid={self.status_error.is_empty()} />
                </Field>
                <Field label="Owner" help={&self.owner_error}>
                    <TextInput value={self.owner.clone()} on_change={ctx.link().callback(TicketMsg::UpdateOwner)} valid={self.owner_error.is_empty()} base_classes="input is-hidden" />
                    <div class={classes!(self.get_dropdown_classes())}>
                        <div class="dropdown-trigger">
                            <TextInput value={self.user_search.clone()} on_change={ctx.link().callback(TicketMsg::SearchUser)} valid={self.owner_error.is_empty()}
                                on_focus={move |_| on_search_focus.emit(true)} on_blur={move |_| on_search_blur.emit(false)} />
                        </div>
                        <div class="dropdown-menu" role="menu">
                            <div class="dropdown-content">
                                { for users }
                            </div>
                        </div>
                    </div>
                </Field>
            </>
        }
    }

    fn submit_button(&self, ctx: &Context<Self>) -> Html {
        let on_submit_pressed = |_: MouseEvent| TicketMsg::Submit();
        html! {
            <button class="button is-link" onmouseup={ctx.link().callback(on_submit_pressed)}>{ "Submit" }</button>
        }
    }

    fn cancel_button(&self, ctx: &Context<Self>) -> Html {
        let on_cancel_pressed = |_: MouseEvent| TicketMsg::Cancel();
        html! {
            <button class="button is-link is-light" onmouseup={ctx.link().callback(on_cancel_pressed)}>{ "Cancel" }</button>
        }
    }

    fn get_project_id(&self) -> IString {
        self.ticket
            .project_id
            .or(self.project_id)
            .map_or(IString::from(""), |id| IString::from(format!("{}", id)))
    }

    fn get_priorities(&self) -> IArray<IString> {
        Priority::iter()
            .map(|v| IString::from(TicketPriority(v).to_string()))
            .collect::<IArray<IString>>()
    }

    fn get_statuses(&self) -> IArray<IString> {
        TicketStatus::iter()
            .map(|v| IString::from(v.to_string()))
            .collect::<IArray<IString>>()
    }

    fn get_dropdown_classes(&self) -> String {
        let mut classes = vec!["dropdown"];
        if self.dropdown_enabled && !self.user_list.is_empty() {
            classes.push("is-active");
        }
        classes.join(" ")
    }

    fn update_errors<E>(&mut self, errors: E)
    where
        E: ValidationMessagesTrait,
    {
        self.common_error = errors.get_common_messages();
        self.title_error = errors.get_property_messages("title");
        self.description_error = errors.get_property_messages("description");
        self.project_error = errors.get_property_messages("project_id");
        self.priority_error = errors.get_property_messages("priority");
        self.status_error = errors.get_property_messages("status");
        self.owner_error = errors.get_property_messages("user_id");
    }
}
