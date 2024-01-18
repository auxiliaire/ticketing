use crate::{components::forms::login_form::LoginForm, services::auth_service::AuthService};
use shared::{api::error::error_response::ErrorResponse, dtos::login_dto::LoginDto};
use yew::prelude::*;

pub enum LoginMsg {
    Submitted((LoginDto, Callback<ErrorResponse>)),
    TokenCreated(LoginDto),
}

pub struct LoginPage {}
impl Component for LoginPage {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMsg::Submitted((creds, callback_error)) => {
                log::debug!("Submitted: {}", creds.username);
                AuthService::authenticate(
                    creds,
                    ctx.link().callback(LoginMsg::TokenCreated),
                    callback_error,
                );
            }
            LoginMsg::TokenCreated(creds) => {
                log::debug!("Created: {}", creds.username);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <section class="hero">
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">{ "Login" }</h1>
                            <h2 class="subtitle">
                                { "Please provide credentials to log in to the application" }
                            </h2>
                        </div>
                    </div>
                </section>
                <div class="section">
                    <LoginForm onsubmit={ctx.link().callback(LoginMsg::Submitted)} />
                </div>
            </div>
        }
    }
}
