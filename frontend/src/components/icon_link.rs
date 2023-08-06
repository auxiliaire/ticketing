use crate::components::component_helper::get_icon_classes;
use implicit_clone::unsync::IString;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IconLinkData<R>
where
    R: Routable,
{
    pub icon: IString,
    pub to: R,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props<R>
where
    R: Routable,
{
    pub data: Option<IconLinkData<R>>,
}

pub struct IconLink<R>
where
    R: Routable,
{
    data: Option<IconLinkData<R>>,
}
impl<R> Component for IconLink<R>
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
            Some(IconLinkData { icon, to }) => {
                html! {
                    <Link<R> classes={classes!("icon", "is-small")} {to}>
                        <span class={get_icon_classes(icon)}></span>
                    </Link<R>>
                }
            }
            None => html! {},
        }
    }
}
