use implicit_clone::sync::IString;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonLinkData<R>
where
    R: Routable,
{
    pub label: IString,
    pub to: R,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props<R>
where
    R: Routable,
{
    pub data: Option<ButtonLinkData<R>>,
}

pub struct ButtonLink<R>
where
    R: Routable,
{
    data: Option<ButtonLinkData<R>>,
}
impl<R> Component for ButtonLink<R>
where
    R: Routable + 'static,
{
    type Message = ();
    type Properties = Props<R>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            data: ctx.props().data.clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.data = ctx.props().data.clone();
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match self.data.clone() {
            Some(ButtonLinkData { label, to }) => {
                html! {
                    <Link<R> classes={classes!("button", "is-small", "is-info")} {to}>
                        { label }
                    </Link<R>>
                }
            }
            None => html! {
                <span class={classes!("button", "is-small", "is-static")}>{ "None" }</span>
            },
        }
    }
}
