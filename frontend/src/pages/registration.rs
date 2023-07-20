use crate::{components::forms::registration_form::RegistrationForm, Route};
use frontend::api::user::UserApi;
use shared::{api::error::error_response::ErrorResponse, dtos::user::User as UserDto};
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

pub enum UserMsg {
    Submitted((UserDto, Callback<ErrorResponse>)),
    Created(UserDto),
}

pub struct Registration {}
impl Component for Registration {
    type Message = UserMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserMsg::Submitted((user, callback_error)) => {
                log::debug!("Submitted: {}", user);
                UserApi::create(user, ctx.link().callback(UserMsg::Created), callback_error);
            }
            UserMsg::Created(user) => {
                log::debug!("Created: {}", user);
                let navigator = ctx.link().navigator().unwrap();
                navigator.replace(&Route::User {
                    id: user.id.unwrap(),
                });
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
                            <h1 class="title">{ "Registration" }</h1>
                            <h2 class="subtitle">
                                { "Here you can register to use the application" }
                            </h2>
                        </div>
                    </div>
                </section>
                <div class="section">
                    <RegistrationForm onsubmit={ctx.link().callback(UserMsg::Submitted)} />
                </div>
            </div>
        }
    }
}
