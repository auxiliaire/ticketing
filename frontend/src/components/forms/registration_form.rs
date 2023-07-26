use crate::components::bulma::field::Field;
use crate::components::html::select::Select;
use crate::components::html::text_input::TextInput;
use implicit_clone::unsync::{IArray, IString};
use serde_valid::validation::{Error, Errors, ObjectErrors, PropertyErrorsMap};
use serde_valid::Validate;
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::user::User as UserDto;
use shared::validation::is_empty::IsEmpty;
use shared::validation::user::{OptionUserRole, UserRole, UserValidation};
use shared::validation::validation_messages::{
    ErrorsWrapper, IValidationMessages, ValidationMessagesTrait,
};
use std::str::FromStr;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub onsubmit: Callback<(UserDto, Callback<ErrorResponse>)>,
}

pub enum RegistrationMsg {
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
    password_repeat: IString,
    on_submit: Callback<(UserDto, Callback<ErrorResponse>)>,
    common_error: IValidationMessages,
    name_error: IValidationMessages,
    password_error: IValidationMessages,
    role_error: IValidationMessages,
}
impl Component for RegistrationForm {
    type Message = RegistrationMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user: UserDto::default(),
            password_repeat: IString::from(""),
            on_submit: ctx.props().onsubmit.to_owned(),
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
            RegistrationMsg::UpdateName(name) => {
                self.user.name = String::from(name.as_str());
            }
            RegistrationMsg::UpdatePassword(password) => {
                log::debug!("password update");
                self.user.password = String::from(password.as_str());
            }
            RegistrationMsg::UpdatePasswordVerification(password) => {
                self.password_repeat = password;
            }
            RegistrationMsg::UpdateRole(role) => {
                self.user.role = UserRole::from_str(role.as_str()).ok();
            }
            RegistrationMsg::Submit() => {
                let result = self.validate();
                match result {
                    Ok(_) => self.on_submit.emit((
                        self.user.clone(),
                        ctx.link().callback(RegistrationMsg::UpdateErrors),
                    )),
                    Err(e) => self.update_errors(ErrorsWrapper(e)),
                }
            }
            RegistrationMsg::UpdateErrors(error_response) => {
                log::debug!("Error response: {}", error_response);
                if let Some(errors) = error_response.details {
                    self.update_errors(errors);
                }
            }
            RegistrationMsg::Cancel() => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.back();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_cancel_pressed = |_: MouseEvent| RegistrationMsg::Cancel();
        let on_submit_pressed = |_: MouseEvent| RegistrationMsg::Submit();
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
                        <TextInput value={self.user.name.clone()} on_change={ctx.link().callback(RegistrationMsg::UpdateName)} valid={self.name_error.is_empty()} />
                    </Field>
                    <Field label="Password" help={&self.password_error}>
                        <TextInput value={self.user.password.clone()} on_change={ctx.link().callback(RegistrationMsg::UpdatePassword)} mask={true} valid={self.password_error.is_empty()} />
                    </Field>
                    <Field label="Password Verification">
                        <TextInput value={self.password_repeat.clone()} on_change={ctx.link().callback(RegistrationMsg::UpdatePasswordVerification)} mask={true} />
                    </Field>
                    <Field label="Role" help={&self.role_error}>
                        <Select value={OptionUserRole(self.user.role).to_string()} options={self.get_roles()} on_change={ctx.link().callback(RegistrationMsg::UpdateRole)} valid={self.role_error.is_empty()} placeholder="Choose role" />
                    </Field>
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

impl RegistrationForm {
    fn validate(&self) -> Result<(), Errors> {
        let user_valid = self.user.validate();
        let passwords_matching = UserValidation::are_passwords_matching(
            self.user.password.as_str(),
            self.password_repeat.as_str(),
        );
        use serde_valid::validation::Errors::Object as ErrorsObject;
        match (user_valid, passwords_matching) {
            (Ok(_), Ok(_)) => Ok(()),
            (Ok(_), Err(e)) => {
                let errors: Vec<Error> = vec![e];
                Err(ErrorsObject(ObjectErrors::new(
                    errors,
                    PropertyErrorsMap::new(),
                )))
            }
            (Err(e), Ok(_)) => Err(e),
            (Err(ErrorsObject(e1)), Err(e2)) => {
                let errors: Vec<Error> = vec![e2];
                Err(ErrorsObject(ObjectErrors::new(errors, e1.properties)))
            }
            (_, Err(e2)) => Err(ErrorsObject(ObjectErrors::new(
                vec![e2],
                PropertyErrorsMap::new(),
            ))),
        }
    }

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
