use crate::components::bulma::field::Field;
use crate::components::html::select::Select;
use crate::components::html::text_input::TextInput;
use implicit_clone::sync::{IArray, IString};
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::user::User as UserDto;
use shared::validation::is_empty::IsEmpty;
use shared::validation::user::{OptionUserRole, UserRole};
use shared::validation::validation_messages::{
    ErrorsWrapper, IValidationMessages, ValidationMessagesTrait,
};
use std::str::FromStr;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub on_submit: Callback<(UserDto, Callback<ErrorResponse>)>,
}

pub enum Msg {
    UpdateName(AttrValue),
    UpdatePassword(AttrValue),
    UpdatePasswordVerification(AttrValue),
    UpdateRole(AttrValue),
    Submit(),
    UpdateErrors(ErrorResponse),
    Cancel(),
}

pub struct RegistrationForm {
    user: UserDto,
    on_submit: Callback<(UserDto, Callback<ErrorResponse>)>,
    common_error: IValidationMessages,
    name_error: IValidationMessages,
    password_error: IValidationMessages,
    role_error: IValidationMessages,
}
impl Component for RegistrationForm {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user: UserDto::default(),
            on_submit: ctx.props().on_submit.to_owned(),
            common_error: None,
            name_error: None,
            password_error: None,
            role_error: None,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateName(name) => {
                self.user.name = String::from(name.as_str());
            }
            Msg::UpdatePassword(password) => {
                log::debug!("password update");
                self.user.password = String::from(password.as_str());
            }
            Msg::UpdatePasswordVerification(password) => {
                self.user.password_repeat = String::from(password.as_str());
            }
            Msg::UpdateRole(role) => {
                self.user.role = UserRole::from_str(role.as_str()).ok();
            }
            Msg::Submit() => {
                let result = Result::Ok(()); // self.user.validate();
                match result {
                    Ok(_) => self
                        .on_submit
                        .emit((self.user.clone(), ctx.link().callback(Msg::UpdateErrors))),
                    Err(e) => self.update_errors(ErrorsWrapper(e)),
                }
            }
            Msg::UpdateErrors(error_response) => {
                log::debug!("Error response: {}", error_response);
                if let Some(errors) = error_response.details {
                    self.update_errors(errors);
                }
            }
            Msg::Cancel() => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.back();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_cancel_pressed = |_: MouseEvent| Msg::Cancel();
        let on_submit_pressed = |_: MouseEvent| Msg::Submit();
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
                    <Field label="Name" help={&self.name_error}>
                        <TextInput value={self.user.name.clone()} on_change={ctx.link().callback(Msg::UpdateName)} valid={self.name_error.is_empty()} />
                    </Field>
                    <Field label="Password" help={&self.password_error}>
                        <TextInput value={self.user.password.clone()} on_change={ctx.link().callback(Msg::UpdatePassword)} mask={true} valid={self.password_error.is_empty()} />
                    </Field>
                    <Field label="Password Verification">
                        <TextInput value={self.user.password_repeat.clone()} on_change={ctx.link().callback(Msg::UpdatePasswordVerification)} mask={true} />
                    </Field>
                    <Field label="Role" help={&self.role_error}>
                        <Select value={OptionUserRole(self.user.role).to_string()} options={self.get_roles()} on_change={ctx.link().callback(Msg::UpdateRole)} valid={self.role_error.is_empty()} placeholder="Choose role" />
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

impl RegistrationForm {
    fn get_roles(&self) -> IArray<IString> {
        UserRole::iter()
            .map(|v| IString::from(v.to_string()))
            .collect::<IArray<IString>>()
    }

    fn update_errors<E>(&mut self, errors: E)
    where
        E: ValidationMessagesTrait,
    {
        self.common_error = errors.get_common_messages();
        self.name_error = errors.get_property_messages("name");
        self.password_error = errors.get_property_messages("password");
        self.role_error = errors.get_property_messages("role");
    }
}
