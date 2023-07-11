use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::components::text_input::TextInput;
use shared::dtos::user::User as UserDto;
use shared::validation::user::UserValidation as validation;

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
    password_verification: String,
    on_submit: Callback<UserDto>,
    dirty: bool,
}
impl Component for RegistrationForm {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user: UserDto::default(),
            password_verification: String::from(""),
            on_submit: ctx.props().on_submit.to_owned(),
            dirty: false,
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
                self.password_verification = String::from(password.as_str());
            }
            Msg::UpdateRole(role) => {
                self.user.role = String::from(role.as_str());
            }
            Msg::Submit() => {
                log::debug!("Submit pressed, user: {}", self.user);
                self.dirty = true;
                //if self.is_username_valid() && self.is_password_valid() {
                    self.on_submit.emit(self.user.clone());
                //}
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
                    <div class="field">
                        <label class="label">{ "Name" }</label>
                        <div class="control">
                            <TextInput value={self.user.name.clone()} on_change={ctx.link().callback(Msg::UpdateName)} valid={self.is_username_valid()} />
                        </div>
                    </div>
                    <div class="field">
                        <label class="label">{ "Password" }</label>
                        <div class="control">
                            <TextInput value={self.user.password.clone()} on_change={ctx.link().callback(Msg::UpdatePassword)} mask={true} valid={self.is_password_valid()} />
                        </div>
                    </div>
                    <div class="field">
                        <label class="label">{ "Password Verification" }</label>
                        <div class="control">
                            <TextInput value={self.password_verification.clone()} on_change={ctx.link().callback(Msg::UpdatePasswordVerification)} mask={true} valid={self.is_password_valid()} />
                        </div>
                    </div>
                    <div class="field">
                        <label class="label">{ "Role" }</label>
                        <div class="control">
                            <TextInput value={self.user.role.clone()} on_change={ctx.link().callback(Msg::UpdateRole)} />
                        </div>
                    </div>
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
    pub fn is_username_valid(&self) -> bool {
        !self.dirty || validation::is_username_valid(self.user.name.as_str())
    }

    pub fn is_password_valid(&self) -> bool {
        !self.dirty || ((self.user.password == self.password_verification) && (validation::is_password_valid(self.user.password.as_str())))
    }
}
