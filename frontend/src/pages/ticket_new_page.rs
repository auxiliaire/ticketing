use crate::{
    app_state::AppStateContext, components::forms::ticket_form::TicketForm, route::Route,
    services::ticket_service::TicketService,
};
use shared::{
    api::error::error_response::ErrorResponse,
    dtos::{login_dto::LoginDto, ticket_dto::TicketDto},
};
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

pub enum TicketMsg {
    ContextChanged(AppStateContext),
    Submitted((TicketDto, Callback<ErrorResponse>)),
    Created(TicketDto),
}

pub struct TicketNewPage {
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}

impl Component for TicketNewPage {
    type Message = TicketMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(TicketMsg::ContextChanged))
            .expect("context to be set");
        if app_state.identity.is_none() {
            let navigator = ctx.link().navigator().unwrap();
            navigator.replace(&Route::Login);
        }
        Self {
            app_state,
            _listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TicketMsg::ContextChanged(state) => {
                self.app_state = state;
            }
            TicketMsg::Submitted((ticket, callback_error)) => {
                log::debug!("Submitted: {}", ticket);
                if let Some(LoginDto { token, .. }) = &self.app_state.identity {
                    TicketService::create(
                        token.to_string(),
                        ticket,
                        ctx.link().callback(TicketMsg::Created),
                        callback_error,
                    );
                }
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
