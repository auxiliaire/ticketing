use crate::{
    app_state::{AppState, AppStateContext},
    services::user_service::UserService,
};
use shared::dtos::{
    identity::Identity,
    preferences_dto::{PreferencesDto, Theme},
};
use yew::{prelude::*, Children};

const BODY_CLASS: &str = "body";

#[derive(Debug, PartialEq, Properties)]
pub struct ThemingProps {
    #[prop_or_default]
    pub children: Children,
}

pub enum ThemingMsg {
    ContextChanged(AppStateContext),
    UpdatePreferences(PreferencesDto),
}

pub struct Theming {
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
    initialized: bool,
}

impl Component for Theming {
    type Message = ThemingMsg;
    type Properties = ThemingProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(ThemingMsg::ContextChanged))
            .expect("context to be set");
        Theming::init(&app_state, ctx);
        Self {
            app_state,
            _listener,
            initialized: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ThemingMsg::ContextChanged(state) => {
                self.app_state = state;
                if !self.initialized {
                    Theming::init(&self.app_state, ctx);
                }
            }
            ThemingMsg::UpdatePreferences(preferences) => {
                self.initialized = true;
                AppState::update_preferences(&self.app_state, Some(preferences));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!(self.get_classes())}>
                { ctx.props().children.clone() }
            </div>
        }
    }
}

impl Theming {
    fn init(app_state: &AppStateContext, ctx: &Context<Self>) {
        if let Some(Identity { token, .. }) = &app_state.identity {
            UserService::fetch_preferences(
                token.to_string(),
                ctx.link().callback(ThemingMsg::UpdatePreferences),
            );
        }
    }

    fn get_classes(&self) -> String {
        let mut classes = vec![BODY_CLASS];
        let theme: String = self.get_theme().unwrap_or_default().to_string();
        if !theme.is_empty() {
            classes.push(theme.as_str());
        }
        classes.join(" ")
    }

    fn get_theme(&self) -> Option<Theme> {
        self.app_state
            .preferences
            .clone()
            .and_then(|prefs| prefs.theme)
    }
}
