use crate::dialog::Dialog;
use shared::dtos::login_dto::LoginDto;
use std::rc::Rc;
use yew::{
    function_component, html, use_reducer, ContextProvider, Html, Properties, Reducible,
    UseReducerHandle,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AppState {
    pub dialog: Rc<Dialog>,
    pub navbar_active: bool,
    pub identity: Option<LoginDto>,
}

impl AppState {
    pub fn update_dialog(ctx: &AppStateContext, dialog: Rc<Dialog>) {
        ctx.dispatch(AppState {
            dialog: dialog.clone(),
            navbar_active: ctx.navbar_active,
            identity: ctx.identity.clone(),
        });
    }

    pub fn close_dialog(ctx: &AppStateContext) {
        ctx.dispatch(AppState {
            // This resets the dialog (which is closed by default):
            dialog: Dialog::default().into(),
            navbar_active: ctx.navbar_active,
            identity: ctx.identity.clone(),
        });
    }

    pub fn toggle_navbar(ctx: &AppStateContext) {
        ctx.dispatch(AppState {
            dialog: ctx.dialog.clone(),
            navbar_active: !ctx.navbar_active,
            identity: ctx.identity.clone(),
        });
    }

    pub fn update_identity_and_close_dialog(ctx: &AppStateContext, identity: Option<LoginDto>) {
        ctx.dispatch(AppState {
            dialog: Dialog::default().into(),
            navbar_active: ctx.navbar_active,
            identity: identity.clone(),
        });
    }
}

impl Reducible for AppState {
    type Action = AppState;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.clone().into()
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;

#[derive(Debug, PartialEq, Properties)]
pub struct AppStateProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn AppStateProvider(props: &AppStateProviderProps) -> Html {
    let context = use_reducer(AppState::default);

    html! {
        <ContextProvider<AppStateContext> {context}>
            { props.children.clone() }
        </ContextProvider<AppStateContext>>
    }
}
