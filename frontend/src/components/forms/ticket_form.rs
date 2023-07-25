use crate::components::bulma::field::Field;
use crate::components::html::select::Select;
use crate::components::html::text_input::TextInput;
use frontend::api::user::UserApi;
use gloo_timers::callback::Timeout;
use implicit_clone::sync::{IArray, IString};
use implicit_clone::unsync::IString as UIString;
use serde_valid::Validate;
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::ticket::Ticket as TicketDto;
use shared::dtos::user::User as UserDto;
use shared::validation::is_empty::IsEmpty;
use shared::validation::ticket::TicketStatus;
use shared::validation::validation_messages::{
    ErrorsWrapper, IValidationMessages, ValidationMessagesTrait,
};
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
    UpdateTitle(AttrValue),
    UpdateDescription(AttrValue),
    UpdateStatus(AttrValue),
    UpdateOwner(AttrValue),
    UpdateUserId((u64, UIString)),
    SearchUser(AttrValue),
    ToggleSearchDropdownDelayed(bool),
    ToggleSearchDropdown(bool),
    FetchedUsers(Vec<UserDto>),
    Submit(),
    UpdateErrors(ErrorResponse),
    Cancel(),
}

pub struct TicketForm {
    ticket: TicketDto,
    project_id: Option<u64>,
    owner: UIString,
    user_search: UIString,
    search_timeout: Option<Timeout>,
    dropdown_enabled: bool,
    user_list: IArray<(u64, UIString)>,
    on_submit: Callback<(TicketDto, Callback<ErrorResponse>)>,
    common_error: IValidationMessages,
    title_error: IValidationMessages,
    description_error: IValidationMessages,
    project_error: IValidationMessages,
    owner_error: IValidationMessages,
    status_error: IValidationMessages,
}
impl Component for TicketForm {
    type Message = TicketMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            ticket: TicketDto::default(),
            project_id: ctx.props().projectid,
            owner: UIString::from(""),
            user_search: UIString::from(""),
            search_timeout: None,
            dropdown_enabled: false,
            user_list: IArray::from(vec![]),
            on_submit: ctx.props().onsubmit.to_owned(),
            common_error: None,
            title_error: None,
            description_error: None,
            project_error: None,
            owner_error: None,
            status_error: None,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TicketMsg::UpdateTitle(title) => {
                self.ticket.title = String::from(title.as_str());
            }
            TicketMsg::UpdateDescription(description) => {
                self.ticket.description = String::from(description.as_str());
            }
            TicketMsg::UpdateStatus(value) => {
                if let Ok(status) = TicketStatus::from_str(value.as_str()) {
                    self.ticket.status = status;
                }
            }
            TicketMsg::UpdateOwner(value) => {
                self.owner = value;
                if let Ok(v) = self.owner.as_str().parse::<u64>() {
                    self.ticket.user_id = Some(v);
                }
            }
            TicketMsg::UpdateUserId((id, name)) => {
                self.owner = UIString::from(format!("{}", id));
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
                    UserApi::fetch_all(Some(q), fetch_callback)
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
                    v.push((u.id.unwrap(), UIString::from(u.name.clone())));
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
            TicketMsg::Cancel() => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.back();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_cancel_pressed = |_: MouseEvent| TicketMsg::Cancel();
        let on_submit_pressed = |_: MouseEvent| TicketMsg::Submit();

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
            <div class="card">
                <div class="card-content">
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
                    <Field label="Summary" help={&self.title_error}>
                        <TextInput value={self.ticket.title.clone()} on_change={ctx.link().callback(TicketMsg::UpdateTitle)} valid={self.title_error.is_empty()} />
                    </Field>
                    <Field label="Description" help={&self.description_error}>
                        <TextInput value={self.ticket.description.clone()} on_change={ctx.link().callback(TicketMsg::UpdateDescription)} valid={self.description_error.is_empty()} />
                    </Field>
                    <Field label="Project" help={&self.project_error}>
                        <TextInput value={self.get_project_id()} on_change={ctx.link().callback(TicketMsg::UpdateDescription)} valid={self.project_error.is_empty()} />
                    </Field>
                    <Field label="Status" help={&self.status_error}>
                        <Select value={self.ticket.status.to_string()} options={self.get_statuses()} on_change={ctx.link().callback(TicketMsg::UpdateStatus)} valid={self.status_error.is_empty()} placeholder="Select status" />
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
                <footer class="card-footer">
                    <div class="card-footer-item">
                        <div class="field is-grouped">
                            <div class="control">
                                <button class="button is-link" onmouseup={ctx.link().callback(on_submit_pressed)}>{ "Submit" }</button>
                            </div>
                            <div class="control">
                                <button class="button is-link is-light" onmouseup={ctx.link().callback(on_cancel_pressed)}>{ "Cancel" }</button>
                            </div>
                        </div>
                    </div>
                </footer>
            </div>
        }
    }
}

impl TicketForm {
    fn get_project_id(&self) -> UIString {
        self.ticket.project_id.or(self.project_id).map_or(UIString::from(""), |id| UIString::from(format!("{}", id)))
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
        self.title_error = errors.get_property_messages("summary");
        self.description_error = errors.get_property_messages("ts_seconds_option");
        self.owner_error = errors.get_property_messages("user_id");
    }
}
