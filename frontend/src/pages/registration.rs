use yew::prelude::*;

use crate::components::registration_form::RegistrationForm;
use shared::dtos::user::User as UserDto;

pub enum Msg {
    UserSubmitted(UserDto),
}

pub struct Registration {}
impl Component for Registration {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UserSubmitted(user) => {
                log::debug!("Submitted: {}", user);
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
                    <RegistrationForm on_submit={ctx.link().callback(Msg::UserSubmitted)} />
                </div>
            </div>
        }
    }
}
