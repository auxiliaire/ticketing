use crate::{dialog::Dialog, route::Route};
use shared::dtos::{
    identity::Identity,
    preferences_dto::{PreferencesDto, Theme},
};
use std::rc::Rc;
use yew::{
    function_component, html, use_reducer, ContextProvider, Html, Properties, Reducible,
    UseReducerHandle,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AppState {
    pub dialog: Rc<Dialog>,
    pub navbar_active: bool,
    pub identity: Option<Identity>,
    pub referer: Option<Route>,
    pub preferences: Option<PreferencesDto>,
}

impl AppState {
    pub fn update_dialog(ctx: &AppStateContext, dialog: Rc<Dialog>) {
        ctx.dispatch(AppStateChange::UpdateDialog(dialog));
    }

    pub fn close_dialog(ctx: &AppStateContext) {
        ctx.dispatch(AppStateChange::CloseDialog);
    }

    pub fn toggle_navbar(ctx: &AppStateContext) {
        ctx.dispatch(AppStateChange::ToggleNavbar);
    }

    pub fn update_identity(ctx: &AppStateContext, identity: Option<Identity>) {
        ctx.dispatch(AppStateChange::UpdateIdentity(identity));
    }

    pub fn update_identity_and_close_dialog(ctx: &AppStateContext, identity: Option<Identity>) {
        ctx.dispatch(AppStateChange::UpdateIdentityAndCloseDialog(identity));
    }

    pub fn update_referer(ctx: &AppStateContext, referer: Option<Route>) {
        ctx.dispatch(AppStateChange::UpdateReferer(referer));
    }

    pub fn update_preferences(ctx: &AppStateContext, preferences: Option<PreferencesDto>) {
        ctx.dispatch(AppStateChange::UpdatePreferences(preferences));
    }

    pub fn switch_theme(ctx: &AppStateContext) {
        ctx.dispatch(AppStateChange::SwitchTheme);
    }
}

pub enum AppStateChange {
    UpdateDialog(Rc<Dialog>),
    CloseDialog,
    ToggleNavbar,
    UpdateIdentity(Option<Identity>),
    UpdateIdentityAndCloseDialog(Option<Identity>),
    UpdateReferer(Option<Route>),
    UpdatePreferences(Option<PreferencesDto>),
    SwitchTheme,
}

impl Reducible for AppState {
    type Action = AppStateChange;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppStateChange::UpdateDialog(dialog) => AppState {
                dialog: dialog.clone(),
                navbar_active: self.navbar_active,
                identity: self.identity.clone(),
                referer: self.referer.clone(),
                preferences: self.preferences.clone(),
            },
            AppStateChange::CloseDialog => AppState {
                // This resets the dialog (which is closed by default):
                dialog: Dialog::default().into(),
                navbar_active: self.navbar_active,
                identity: self.identity.clone(),
                referer: self.referer.clone(),
                preferences: self.preferences.clone(),
            },
            AppStateChange::ToggleNavbar => AppState {
                dialog: self.dialog.clone(),
                navbar_active: !self.navbar_active,
                identity: self.identity.clone(),
                referer: self.referer.clone(),
                preferences: self.preferences.clone(),
            },
            AppStateChange::UpdateIdentity(identity) => AppState {
                dialog: self.dialog.clone(),
                navbar_active: self.navbar_active,
                identity: identity.clone(),
                referer: self.referer.clone(),
                preferences: self.preferences.clone(),
            },
            AppStateChange::UpdateIdentityAndCloseDialog(identity) => AppState {
                dialog: Dialog::default().into(),
                navbar_active: self.navbar_active,
                identity: identity.clone(),
                referer: self.referer.clone(),
                preferences: self.preferences.clone(),
            },
            AppStateChange::UpdateReferer(referer) => AppState {
                dialog: self.dialog.clone(),
                navbar_active: self.navbar_active,
                identity: self.identity.clone(),
                referer: referer.clone(),
                preferences: self.preferences.clone(),
            },
            AppStateChange::UpdatePreferences(preferences) => AppState {
                dialog: self.dialog.clone(),
                navbar_active: self.navbar_active,
                identity: self.identity.clone(),
                referer: self.referer.clone(),
                preferences,
            },
            AppStateChange::SwitchTheme => AppState {
                dialog: self.dialog.clone(),
                navbar_active: self.navbar_active,
                identity: self.identity.clone(),
                referer: self.referer.clone(),
                preferences: Some(PreferencesDto {
                    theme: Some(Theme::flip(
                        self.preferences
                            .clone()
                            .unwrap_or_default()
                            .theme
                            .unwrap_or_default(),
                    )),
                    ..self.preferences.clone().unwrap_or_default()
                }),
            },
        }
        .into()
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
