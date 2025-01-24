use crate::app_state::AppStateContext;
use crate::components::bulma::field::Field;
use crate::components::dialogs::dialog_context::DialogContext;
use crate::components::html::checkbox::Checkbox;
use crate::components::html::select::Select;
use implicit_clone::unsync::{IArray, IString};
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::preferences_dto::{PreferencesDto, Theme};
use shared::validation::is_empty::IsEmpty;
use shared::validation::validation_messages::{
    ErrorsWrapper, IValidationMessages, ValidationMessagesTrait,
};
use std::rc::Rc;
use std::str::FromStr;
use strum::IntoEnumIterator;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub onsubmit: Callback<(PreferencesDto, Callback<ErrorResponse>)>,
}

pub enum PreferencesMsg {
    ContextChanged(AppStateContext),
    DialogContextChanged(Rc<DialogContext>),
    UpdateTheme(AttrValue),
    UpdateMfa(bool),
    UpdateProjectNotification(bool),
    UpdateTicketNotification(bool),
    Submit(),
    UpdateErrors(ErrorResponse),
}

pub struct PreferencesForm {
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
    dialog_context: Option<Rc<DialogContext>>,
    preferences: PreferencesDto,
    on_submit: Callback<(PreferencesDto, Callback<ErrorResponse>)>,
    common_error: IValidationMessages,
    theme_error: IValidationMessages,
    mfa_error: IValidationMessages,
    dirty: bool,
}
impl Component for PreferencesForm {
    type Message = PreferencesMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(PreferencesMsg::ContextChanged))
            .expect("context to be set");
        let preferences =
            <std::option::Option<shared::dtos::preferences_dto::PreferencesDto> as Clone>::clone(
                &app_state.preferences,
            )
            .unwrap_or_default();
        let option_dialog_context = ctx.link().context::<Rc<DialogContext>>(
            ctx.link().callback(PreferencesMsg::DialogContextChanged),
        );
        let dialog_context = option_dialog_context.map(|(context, _listener)| context);
        Self {
            app_state,
            _listener,
            dialog_context,
            preferences,
            on_submit: ctx.props().onsubmit.to_owned(),
            common_error: None,
            theme_error: None,
            mfa_error: None,
            dirty: false,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PreferencesMsg::ContextChanged(state) => {
                self.app_state = state;
                self.preferences = <std::option::Option<
                    shared::dtos::preferences_dto::PreferencesDto,
                > as Clone>::clone(&self.app_state.preferences)
                .unwrap_or_default();
                log::debug!(">>> Preferences: {:?}", self.preferences);
                self.dirty = false;
            }
            PreferencesMsg::DialogContextChanged(context) => {
                self.dialog_context = Some(context);
            }
            PreferencesMsg::UpdateTheme(theme) => {
                self.preferences.theme = Theme::from_str(&theme).ok();
                self.dirty = true;
            }
            PreferencesMsg::UpdateMfa(mfa) => {
                self.preferences.mfa = Some(mfa);
                self.dirty = true;
            }
            PreferencesMsg::UpdateProjectNotification(active) => {
                let mut notifications = <std::option::Option<
                    shared::dtos::preferences_dto::Notifications,
                > as Clone>::clone(
                    &self.preferences.notifications
                )
                .unwrap_or_default()
                .clone();
                notifications.projects = active;
                self.preferences.notifications = Some(notifications);
                self.dirty = true;
            }
            PreferencesMsg::UpdateTicketNotification(active) => {
                let mut notifications = <std::option::Option<
                    shared::dtos::preferences_dto::Notifications,
                > as Clone>::clone(
                    &self.preferences.notifications
                )
                .unwrap_or_default()
                .clone();
                notifications.tickets = active;
                self.preferences.notifications = Some(notifications);
                self.dirty = true;
            }
            PreferencesMsg::Submit() => {
                // Server validation only due to time panic in Wasm:
                let result = Ok(true); // self.project.validate();
                match result {
                    Ok(_) => self.on_submit.emit((
                        self.preferences.clone(),
                        ctx.link().callback(PreferencesMsg::UpdateErrors),
                    )),
                    Err(e) => self.update_errors(ErrorsWrapper(e)),
                }
            }
            PreferencesMsg::UpdateErrors(error_response) => {
                log::debug!("Error response: {}", error_response);
                if let Some(errors) = error_response.details {
                    self.update_errors(errors);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_submit_pressed = |_: MouseEvent| PreferencesMsg::Submit();

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
                    <Field label="Default Theme" help={&self.theme_error} class="is-one-third">
                        <Select value={self.get_theme()} options={self.get_themes()} on_change={ctx.link().callback(PreferencesMsg::UpdateTheme)} valid={self.theme_error.is_empty()} />
                    </Field>
                    <div class="field pt-2">
                        <p class="control">
                            <label class="checkbox">
                                <Checkbox checked={self.preferences.mfa.unwrap_or_default()} on_change={ctx.link().callback(PreferencesMsg::UpdateMfa)} />
                                <b>{ " Multi-factor Authentication" }</b>
                            </label>
                        </p>
                    </div>
                    <div class="field pt-2">
                        <label class="label">{ "Notifications" }</label>
                        <p class="control">
                            <label class="checkbox">
                                <Checkbox checked={self.preferences.notifications.clone().unwrap_or_default().projects} on_change={ctx.link().callback(PreferencesMsg::UpdateProjectNotification)} />
                                <b>{ " Project updates" }</b>
                            </label>
                        </p>
                        <p class="control">
                            <label class="checkbox">
                                <Checkbox checked={self.preferences.notifications.clone().unwrap_or_default().tickets} on_change={ctx.link().callback(PreferencesMsg::UpdateTicketNotification)} />
                                <b>{ " Ticket updates" }</b>
                            </label>
                        </p>
                    </div>
                </div>
                <footer class="card-footer">
                    <div class="card-content">
                        <div class="field is-grouped">
                            <div class="control">
                                <button class="button is-link" onmouseup={ctx.link().callback(on_submit_pressed)}>{ "Save" }</button>
                                {
                                    if self.dirty {
                                        html! {
                                            <span class="icon is-medium has-text-warning">
                                                <i class="fa-solid fa-circle-exclamation"></i>
                                            </span>
                                        }
                                    } else {
                                        html! {
                                            <span class="icon is-medium has-text-success">
                                                <i class="fa-solid fa-circle-check"></i>
                                            </span>
                                        }
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </footer>
            </div>
        }
    }
}

impl PreferencesForm {
    fn get_theme(&self) -> IString {
        IString::from(
            self.preferences
                .theme
                .clone()
                .unwrap_or_default()
                .to_string(),
        )
    }

    fn get_themes(&self) -> IArray<IString> {
        Theme::iter()
            .map(|v| IString::from(v.to_string()))
            .collect::<IArray<IString>>()
    }

    fn update_errors<E>(&mut self, errors: E)
    where
        E: ValidationMessagesTrait,
    {
        self.common_error = errors.get_common_messages();
        self.theme_error = errors.get_property_messages("theme");
        self.mfa_error = errors.get_property_messages("mfa");
    }
}
