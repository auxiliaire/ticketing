use implicit_clone::sync::{IArray, IString};
use serde_valid::Validate;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::components::bulma::field::Field;
use crate::components::text_input::TextInput;
use shared::dtos::user::User as UserDto;
use shared::validation::error_messages::{ErrorMessages, ErrorsWrapper};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub on_submit: Callback<UserDto>,
}

pub enum Msg {
    UpdateName(AttrValue),
    UpdatePassword(AttrValue),
    UpdatePasswordVerification(AttrValue),
    UpdateRole(AttrValue),
    Submit(),
    Cancel(),
}

pub struct RegistrationForm {
    user: UserDto,
    on_submit: Callback<UserDto>,
    dirty: bool,
    common_error: Option<IArray<IString>>,
    name_error: Option<IArray<IString>>,
    password_error: Option<IArray<IString>>,
    role_error: Option<IArray<IString>>,
}
impl Component for RegistrationForm {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user: UserDto::default(),
            on_submit: ctx.props().on_submit.to_owned(),
            dirty: false,
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
                self.user.role = String::from(role.as_str());
            }
            Msg::Submit() => {
                self.dirty = true;
                let result = self.user.validate();
                match result {
                    Ok(_) => self.on_submit.emit(self.user.clone()),
                    Err(e) => {
                        let errors = ErrorsWrapper(e);
                        self.common_error = errors.get_common_messages();
                        self.name_error = errors.get_property_messages("name");
                        self.password_error = errors.get_property_messages("password");
                        self.role_error = errors.get_property_messages("role");
                    }
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
                        <TextInput value={self.user.name.clone()} on_change={ctx.link().callback(Msg::UpdateName)} valid={self.name_error.is_none()} />
                    </Field>
                    <Field label="Password" help={&self.password_error}>
                        <TextInput value={self.user.password.clone()} on_change={ctx.link().callback(Msg::UpdatePassword)} mask={true} valid={self.password_error.is_none()} />
                    </Field>
                    <Field label="Password Verification">
                        <TextInput value={self.user.password_repeat.clone()} on_change={ctx.link().callback(Msg::UpdatePasswordVerification)} mask={true} />
                    </Field>
                    <Field label="Role" help={&self.role_error}>
                        <TextInput value={self.user.role.clone()} on_change={ctx.link().callback(Msg::UpdateRole)} valid={self.role_error.is_none()} />
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

impl RegistrationForm {}
