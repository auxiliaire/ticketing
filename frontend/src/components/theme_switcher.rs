use crate::{
    app_state::{AppState, AppStateContext},
    components::theme_icon::ThemeIcon,
};
use shared::dtos::preferences_dto::Theme;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct ThemeSwitcherProps {}

pub enum ThemeSwitcherMsg {
    ContextChanged(AppStateContext),
    SwitchTheme,
}

pub struct ThemeSwitcher {
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}

impl Component for ThemeSwitcher {
    type Message = ThemeSwitcherMsg;
    type Properties = ThemeSwitcherProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(ThemeSwitcherMsg::ContextChanged))
            .expect("context to be set");

        Self {
            app_state,
            _listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ThemeSwitcherMsg::ContextChanged(state) => {
                self.app_state = state;
            }
            ThemeSwitcherMsg::SwitchTheme => {
                AppState::switch_theme(&self.app_state);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="p-3 has-text-light is-clickable" onclick={ctx.link().callback(|_| ThemeSwitcherMsg::SwitchTheme)}>
                <ThemeIcon dark={self.is_dark()} />
            </div>
        }
    }
}

impl ThemeSwitcher {
    fn is_dark(&self) -> bool {
        <std::option::Option<shared::dtos::preferences_dto::PreferencesDto> as Clone>::clone(
            &self.app_state.clone().preferences,
        )
        .and_then(|prefs| prefs.theme)
        .map(|theme| theme.eq(&Theme::DARK))
        .unwrap_or_default()
    }
}
