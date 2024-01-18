use crate::dialog::Dialog;
use shared::dtos::login_dto::LoginDto;
use std::rc::Rc;
use yew::{
    function_component, html, use_reducer, Callback, ContextProvider, Html, Properties, Reducible,
    UseReducerHandle,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AppState {
    pub update_dialog: Callback<Rc<Dialog>>,
    pub close_dialog: Callback<()>,
    pub navbar_active: bool,
    pub identity: Option<LoginDto>,
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
    pub state: Rc<AppState>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn AppStateProvider(props: &AppStateProviderProps) -> Html {
    let context = use_reducer(|| AppState {
        update_dialog: props.state.update_dialog.clone(),
        close_dialog: props.state.close_dialog.clone(),
        navbar_active: props.state.navbar_active,
        identity: props.state.identity.clone(),
    });

    html! {
        <ContextProvider<AppStateContext> {context}>
            { props.children.clone() }
        </ContextProvider<AppStateContext>>
    }
}
