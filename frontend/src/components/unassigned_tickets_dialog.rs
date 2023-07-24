use frontend::api::ticket::TicketApi;
use shared::dtos::ticket::Ticket as TicketDto;
use std::rc::Rc;
use yew::{html, Component, Context, ContextHandle, Html, Properties};

use crate::AppState;

pub enum Msg {
    UnassignedTickets(Vec<TicketDto>),
    ContextChanged(Rc<AppState>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct UnassignedTicketsDialog {
    tickets: Vec<TicketDto>,
    app_state: Rc<AppState>,
    _listener: ContextHandle<Rc<AppState>>,
}

impl Component for UnassignedTicketsDialog {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        TicketApi::fetch_unassigned(ctx.link().callback(Msg::UnassignedTickets));
        let (app_state, _listener) = ctx
            .link()
            .context::<Rc<AppState>>(ctx.link().callback(Msg::ContextChanged))
            .expect("context to be set");
        Self {
            tickets: vec![],
            app_state,
            _listener,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        TicketApi::fetch_unassigned(ctx.link().callback(Msg::UnassignedTickets));
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UnassignedTickets(tickets) => {
                self.tickets = tickets;
            }
            Msg::ContextChanged(state) => {
                self.app_state = state;
            }
        }
        true
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        let ticket_options = self.tickets.iter().map(
            |TicketDto {
                 id,
                 title,
                 description: _,
                 project_id: _,
                 status: _,
                 user_id: _,
             }| {
                match id {
                    Some(_id) => html! { <option>{title}</option> },
                    None => html! {},
                }
            },
        );

        html! {
            <>
                <header class="modal-card-head">
                    <p class="modal-card-title">{ "Select tickets to assign" }</p>
                    <button class="delete" aria-label="close" onclick={self.app_state.close_dialog.reform(move |_| ())}></button>
                </header>
                <section class="modal-card-body">
                    <div class="select is-multiple">
                        <select multiple={true} size="8">
                            { for ticket_options }
                        </select>
                    </div>
                </section>
                <footer class="modal-card-foot">
                    <button class="button is-success">{ "Save changes" }</button>
                    <button class="button" onclick={self.app_state.close_dialog.reform(move |_| ())}>{ "Cancel" }</button>
                </footer>
            </>
        }
    }
}
