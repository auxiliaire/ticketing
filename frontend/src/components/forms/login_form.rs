use crate::components::bulma::field::Field;
use crate::components::dialogs::dialog_context::DialogContext;
use crate::components::html::text_input::TextInput;
use core::convert::From;
use implicit_clone::sync::{IArray, IString};
use serde_valid::validation::{Error, Errors, ObjectErrors};
use serde_valid::Validate;
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::login_dto::LoginDto;
use shared::validation::is_empty::IsEmpty;
use shared::validation::validation_messages::{
    ErrorsWrapper, IValidationMessages, ValidationMessagesTrait,
};
use std::rc::Rc;
use std::sync::Arc;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub onsubmit: Callback<(LoginDto, Callback<ErrorResponse>)>,
}

pub enum LoginMsg {
    DialogContextChanged(Rc<DialogContext>),
    UpdateUsername(AttrValue),
    UpdatePassword(AttrValue),
    Submit(),
    UpdateErrors(ErrorResponse),
    Cancel(),
}

pub struct LoginForm {
    dialog_context: Option<Rc<DialogContext>>,
    creds: LoginDto,
    on_submit: Callback<(LoginDto, Callback<ErrorResponse>)>,
    common_error: IValidationMessages,
}
impl Component for LoginForm {
    type Message = LoginMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let option_dialog_context = ctx
            .link()
            .context::<Rc<DialogContext>>(ctx.link().callback(LoginMsg::DialogContextChanged));
        let dialog_context = option_dialog_context.map(|(context, _listener)| context);
        Self {
            dialog_context,
            creds: LoginDto::default(),
            on_submit: ctx.props().onsubmit.to_owned(),
            common_error: None,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMsg::DialogContextChanged(context) => {
                self.dialog_context = Some(context);
            }
            LoginMsg::UpdateUsername(username) => {
                self.creds.username = String::from(username.as_str());
            }
            LoginMsg::UpdatePassword(password) => {
                log::debug!("password update");
                self.creds.password = String::from(password.as_str());
            }
            LoginMsg::Submit() => {
                let result = self.validate();
                match result {
                    Ok(_) => self.on_submit.emit((
                        self.creds.clone(),
                        ctx.link().callback(LoginMsg::UpdateErrors),
                    )),
                    Err(e) => self.update_errors(ErrorsWrapper(e)),
                }
            }
            LoginMsg::UpdateErrors(error_response) => {
                log::debug!("Error response: {}", error_response);
                if let Some(errors) = error_response.details {
                    self.update_errors(errors);
                } else if !error_response.message.is_empty() {
                    self.update_errors_with_message(error_response.message);
                }
            }
            LoginMsg::Cancel() => match self.dialog_context.clone() {
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
        let on_cancel_pressed = |_: MouseEvent| LoginMsg::Cancel();
        let on_submit_pressed = |_: MouseEvent| LoginMsg::Submit();
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
                    <Field label="Username">
                        <TextInput value={self.creds.username.clone()} on_change={ctx.link().callback(LoginMsg::UpdateUsername)} valid={self.common_error.is_empty()} />
                    </Field>
                    <Field label="Password">
                        <TextInput value={self.creds.password.clone()} on_change={ctx.link().callback(LoginMsg::UpdatePassword)} mask={true} valid={self.common_error.is_empty()} />
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

impl LoginForm {
    fn validate(&self) -> Result<(), Errors> {
        let user_valid = self.creds.validate();
        use serde_valid::validation::Errors::Object as ErrorsObject;
        match user_valid {
            Ok(_) => Ok(()),
            Err(ErrorsObject(e1)) => {
                let errors: Vec<Error> = vec![];
                Err(ErrorsObject(ObjectErrors::new(errors, e1.properties)))
            }
            Err(e) => Err(e),
        }
    }

    fn update_errors<E>(&mut self, errors: E)
    where
        E: ValidationMessagesTrait,
    {
        self.common_error = errors.get_common_messages();
    }

    fn update_errors_with_message(&mut self, msg: String) {
        self.common_error = Some(IArray::<IString>::Rc(
            vec![IString::Rc(Arc::<str>::from(msg.as_str()))].into(),
        ));
    }
}
