use crate::{components::forms::ticket_form::TicketForm, Route};
use frontend::services::ticket_service::TicketService;
use shared::{api::error::error_response::ErrorResponse, dtos::ticket::Ticket as TicketDto};
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

pub enum TicketMsg {
    Submitted((TicketDto, Callback<ErrorResponse>)),
    Created(TicketDto),
}

pub struct TicketNew {}
impl Component for TicketNew {
    type Message = TicketMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TicketMsg::Submitted((ticket, callback_error)) => {
                log::debug!("Submitted: {}", ticket);
                TicketService::create(
                    ticket,
                    ctx.link().callback(TicketMsg::Created),
                    callback_error,
                );
            }
            TicketMsg::Created(ticket) => {
                log::debug!("Created: {}", ticket);
                let navigator = ctx.link().navigator().unwrap();
                match ticket.project_id {
                    Some(id) => {
                        navigator.replace(&Route::Project { id });
                    }
                    None => {
                        navigator.replace(&Route::Ticket {
                            id: ticket.id.unwrap(),
                        });
                    }
                }
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
                            <h1 class="title">{ "Create Ticket" }</h1>
                            <h2 class="subtitle">
                                { "Here you can create a new ticket" }
                            </h2>
                        </div>
                    </div>
                </section>
                <div class="section">
                    <TicketForm onsubmit={ctx.link().callback(TicketMsg::Submitted)} />
                </div>
            </div>
        }
    }
}
