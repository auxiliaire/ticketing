use crate::app_state::AppStateContext;
use yew::prelude::*;

pub enum AppModalMsg {
    ContextChanged(AppStateContext),
}

pub struct AppModal {
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}

impl Component for AppModal {
    type Message = AppModalMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(AppModalMsg::ContextChanged))
            .expect("context to be set");
        Self {
            app_state,
            _listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppModalMsg::ContextChanged(state) => {
                self.app_state = state;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let modal_active = match self.app_state.dialog.active {
            true => "is-active",
            false => "",
        };

        html! {
            <div id="app-modal" class={classes!("modal", modal_active)}>
                <div class="modal-background"></div>
                <div class="modal-card">
                    { self.app_state.dialog.clone().content.clone() }
                </div>
            </div>
        }
    }
}
