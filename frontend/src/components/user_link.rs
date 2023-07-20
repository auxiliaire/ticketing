use crate::Route;
use implicit_clone::sync::IString;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub user: Option<(u64, IString)>,
}

pub struct UserLink {
    user: Option<(u64, IString)>,
}
impl Component for UserLink {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user: ctx.props().user.clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.user = ctx.props().user.clone();
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match self.user.as_ref() {
            Some((id, name)) => {
                html! {
                    <Link<Route> classes={classes!("button", "is-small", "is-info")} to={Route::User { id: *id }}>
                        { (*name).to_string() }
                    </Link<Route>>
                }
            }
            None => html! {
                <span class={classes!("button", "is-small", "is-static")}>{ "None" }</span>
            },
        }
    }
}
