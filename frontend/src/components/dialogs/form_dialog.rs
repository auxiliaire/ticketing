use crate::AppState;
use std::rc::Rc;
use yew::{html, Children, Component, Context, ContextHandle, Html, Properties};

pub enum FormDialogMsg {
    ContextChanged(Rc<AppState>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormDialogProps {
    pub children: Children,
}

pub struct FormDialog {
    app_state: Rc<AppState>,
    _listener: ContextHandle<Rc<AppState>>,
}

impl Component for FormDialog {
    type Message = FormDialogMsg;
    type Properties = FormDialogProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<Rc<AppState>>(ctx.link().callback(FormDialogMsg::ContextChanged))
            .expect("context to be set");
        Self {
            app_state,
            _listener,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FormDialogMsg::ContextChanged(state) => {
                self.app_state = state;
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <>
                <header class="modal-card-head">
                    <p class="modal-card-title">{ "Create new ticket" }</p>
                    <button class="delete" aria-label="close" onclick={self.app_state.close_dialog.reform(move |_| ())}></button>
                </header>
                { for ctx.props().children.iter() }
            </>
        }
    }
}
