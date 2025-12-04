use super::dialog_context::DialogContext;
use crate::app_state::{AppState, AppStateContext};
use crate::helpers::component_helper::to_html;
use implicit_clone::unsync::IString;
use std::rc::Rc;
use yew::{html, Children, Component, Context, ContextProvider, Html, Properties};

pub enum FormDialogMsg {
    ContextChanged(AppStateContext),
    CloseDialog,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormDialogProps {
    pub children: Children,
    pub title: IString,
}

pub struct FormDialog {
    app_state: AppStateContext,
    dialog_context: Rc<DialogContext>,
}

impl Component for FormDialog {
    type Message = FormDialogMsg;
    type Properties = FormDialogProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(FormDialogMsg::ContextChanged))
            .expect("context to be set");
        let dialog_context = Rc::new(DialogContext {
            closehandler: ctx.link().callback(|_| FormDialogMsg::CloseDialog),
        });
        Self {
            app_state,
            dialog_context,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FormDialogMsg::ContextChanged(state) => {
                self.app_state = state;
            }
            FormDialogMsg::CloseDialog => AppState::close_dialog(&self.app_state),
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let context = self.dialog_context.clone();
        html! {
            <>
                <header class="modal-card-head">
                    <p class="modal-card-title">{ to_html(ctx.props().title.clone()) }</p>
                    <button class="delete" aria-label="close" onclick={ctx.link().callback(|_| FormDialogMsg::CloseDialog)}></button>
                </header>
                <ContextProvider<Rc<DialogContext>> {context}>
                    { for ctx.props().children.iter() }
                </ContextProvider<Rc<DialogContext>>>
            </>
        }
    }
}
