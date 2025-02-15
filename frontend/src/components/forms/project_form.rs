use crate::app_state::AppStateContext;
use crate::components::bulma::field::Field;
use crate::components::dialogs::dialog_context::DialogContext;
use crate::components::html::checkbox::Checkbox;
use crate::components::html::date_input::DateInput;
use crate::components::html::text_input::TextInput;
use crate::services::user_service::UserService;
use chrono::{NaiveDate, Utc};
use gloo_timers::callback::Timeout;
use implicit_clone::sync::IArray;
use implicit_clone::unsync::IString;
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::identity::Identity;
use shared::dtos::project_dto::ProjectDto;
use shared::dtos::user_dto::UserDto;
use shared::validation::is_empty::IsEmpty;
use shared::validation::validation_messages::{
    ErrorsWrapper, IValidationMessages, ValidationMessagesTrait,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

const SEARCH_DELAY_MS: u32 = 300;
const DROPDOWN_CLOSE_MS: u32 = 200;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub onsubmit: Callback<(ProjectDto, Callback<ErrorResponse>)>,
}

pub enum ProjectMsg {
    ContextChanged(AppStateContext),
    DialogContextChanged(Rc<DialogContext>),
    UpdateSummary(AttrValue),
    UpdateDeadline(AttrValue),
    UpdateOwner(AttrValue),
    UpdateUserId((IString, IString)),
    UpdateActive(bool),
    SearchUser(AttrValue),
    ToggleSearchDropdownDelayed(bool),
    ToggleSearchDropdown(bool),
    FetchedUsers(Vec<UserDto>),
    Submit(),
    UpdateErrors(ErrorResponse),
    Cancel(),
}

pub struct ProjectForm {
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
    dialog_context: Option<Rc<DialogContext>>,
    project: ProjectDto,
    deadline: IString,
    owner: IString,
    user_search: IString,
    search_timeout: Option<Timeout>,
    dropdown_enabled: bool,
    user_list: IArray<(IString, IString)>,
    on_submit: Callback<(ProjectDto, Callback<ErrorResponse>)>,
    common_error: IValidationMessages,
    summary_error: IValidationMessages,
    deadline_error: IValidationMessages,
    owner_error: IValidationMessages,
}
impl Component for ProjectForm {
    type Message = ProjectMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(ProjectMsg::ContextChanged))
            .expect("context to be set");
        let option_dialog_context = ctx
            .link()
            .context::<Rc<DialogContext>>(ctx.link().callback(ProjectMsg::DialogContextChanged));
        let dialog_context = option_dialog_context.map(|(context, _listener)| context);
        Self {
            app_state,
            _listener,
            dialog_context,
            project: ProjectDto::default(),
            deadline: IString::from(""),
            owner: IString::from(""),
            user_search: IString::from(""),
            search_timeout: None,
            dropdown_enabled: false,
            user_list: IArray::from(vec![]),
            on_submit: ctx.props().onsubmit.to_owned(),
            common_error: None,
            summary_error: None,
            deadline_error: None,
            owner_error: None,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProjectMsg::ContextChanged(state) => {
                self.app_state = state;
            }
            ProjectMsg::DialogContextChanged(context) => {
                self.dialog_context = Some(context);
            }
            ProjectMsg::UpdateSummary(summary) => {
                self.project.summary = String::from(summary.as_str());
            }
            ProjectMsg::UpdateDeadline(deadline) => {
                self.deadline = deadline;
                log::debug!("Trying to parse '{}'", self.deadline.as_str());
                match NaiveDate::parse_from_str(self.deadline.as_str(), "%F") {
                    Ok(d) => {
                        log::debug!("Successfully parsed '{}'", self.deadline.as_str());
                        self.project.deadline = Some(
                            d.and_hms_opt(0, 0, 0)
                                .unwrap()
                                .and_local_timezone(Utc)
                                .unwrap(),
                        );
                    }
                    Err(e) => log::debug!("Parse failed with '{}'", e.to_string()),
                }
            }
            ProjectMsg::UpdateOwner(value) => {
                self.owner = value;
                if let Ok(v) = Uuid::parse_str(self.owner.as_str()) {
                    self.project.user_id = v;
                }
            }
            ProjectMsg::UpdateUserId((id, name)) => {
                self.owner = id.clone();
                match Uuid::parse_str(id.as_str()) {
                    Ok(uuid) => {
                        self.project.user_id = uuid;
                    }
                    Err(e) => {
                        log::error!("Uuid parse error: '{}'", e.to_string());
                    }
                }
                self.user_search = name;
            }
            ProjectMsg::UpdateActive(active) => {
                self.project.active = match active {
                    true => 1,
                    false => 0,
                };
            }
            ProjectMsg::SearchUser(value) => {
                self.user_search = value;
                // We need to throttle the API call to prevent superfluous calls
                let q = self.user_search.clone();
                let fetch_callback = ctx.link().callback(ProjectMsg::FetchedUsers);
                if let Some(timeout) = self.search_timeout.take() {
                    timeout.cancel();
                }
                self.search_timeout =
                    self.app_state
                        .identity
                        .clone()
                        .map(|Identity { token, .. }| {
                            Timeout::new(SEARCH_DELAY_MS, || {
                                UserService::fetch_all(token, Some(q), None, None, fetch_callback)
                            })
                        });
            }
            ProjectMsg::ToggleSearchDropdownDelayed(value) => {
                let toggle_search_dropdown = ctx.link().callback(ProjectMsg::ToggleSearchDropdown);
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
            ProjectMsg::ToggleSearchDropdown(value) => {
                self.dropdown_enabled = value;
            }
            ProjectMsg::FetchedUsers(list) => {
                let mut v = vec![];
                for u in list {
                    v.push((
                        IString::from(u.public_id.unwrap().to_string()),
                        IString::from(u.name.clone()),
                    ));
                }
                self.user_list = IArray::from(v);
            }
            ProjectMsg::Submit() => {
                // Server validation only due to time panic in Wasm:
                let result = Ok(true); // self.project.validate();
                match result {
                    Ok(_) => self.on_submit.emit((
                        self.project.clone(),
                        ctx.link().callback(ProjectMsg::UpdateErrors),
                    )),
                    Err(e) => self.update_errors(ErrorsWrapper(e)),
                }
            }
            ProjectMsg::UpdateErrors(error_response) => {
                log::debug!("Error response: {}", error_response);
                if let Some(errors) = error_response.details {
                    self.update_errors(errors);
                }
            }
            ProjectMsg::Cancel() => match self.dialog_context.clone() {
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
        let on_cancel_pressed = |_: MouseEvent| ProjectMsg::Cancel();
        let on_submit_pressed = |_: MouseEvent| ProjectMsg::Submit();

        let users = self.user_list.iter().map(|t| {
            let select_user = ctx.link().callback(ProjectMsg::UpdateUserId);
            let name = t.1.clone();
            html! {
                <a class="dropdown-item" onclick={move |_| {
                    select_user.emit((t.0.clone(), t.1.clone()));
                }}>{name}</a>
            }
        });

        let on_search_focus = ctx.link().callback(ProjectMsg::ToggleSearchDropdown);
        let on_search_blur = ctx.link().callback(ProjectMsg::ToggleSearchDropdownDelayed);

        html! {
            <div class="card">
                <div class="card-content">
                    if let Some(common_error) = &self.common_error {
                        <p class="help is-danger">
                            <ul>
                            {
                                common_error.iter().map(|message| {
                                    html!{<li>{ html! {message}}</li>}
                                }).collect::<Html>()
                            }
                            </ul>
                        </p>
                    }
                    <Field label="Summary" help={&self.summary_error}>
                        <TextInput value={self.project.summary.clone()} on_change={ctx.link().callback(ProjectMsg::UpdateSummary)} valid={self.summary_error.is_empty()} />
                    </Field>
                    <Field label="Deadline" help={&self.deadline_error}>
                        <div class="field">
                            <div class="control">
                                <DateInput value={self.deadline.clone()} on_change={ctx.link().callback(ProjectMsg::UpdateDeadline)} valid={self.deadline_error.is_empty()} />
                            </div>
                        </div>
                    </Field>
                    <Field label="Owner" help={&self.owner_error}>
                        <TextInput value={self.owner.clone()} on_change={ctx.link().callback(ProjectMsg::UpdateOwner)} valid={self.owner_error.is_empty()} base_classes="input is-hidden" />
                        <div class={classes!(self.get_dropdown_classes())}>
                            <div class="dropdown-trigger">
                                <TextInput value={self.user_search.clone()} on_change={ctx.link().callback(ProjectMsg::SearchUser)} valid={self.owner_error.is_empty()}
                                    on_focus={move |_| on_search_focus.emit(true)} on_blur={move |_| on_search_blur.emit(false)} />
                            </div>
                            <div class="dropdown-menu" role="menu">
                                <div class="dropdown-content">
                                    { for users }
                                </div>
                            </div>
                        </div>
                    </Field>
                    <div class="field">
                        <p class="control">
                            <label class="checkbox">
                                <Checkbox checked={self.project.active == 1} on_change={ctx.link().callback(ProjectMsg::UpdateActive)} />
                                <b>{ " Active" }</b>
                            </label>
                        </p>
                    </div>
                </div>
                <footer class="card-footer">
                    <div class="card-content">
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

impl ProjectForm {
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
        self.summary_error = errors.get_property_messages("summary");
        self.deadline_error = errors.get_property_messages("ts_seconds_option");
        self.owner_error = errors.get_property_messages("user_id");
    }
}
