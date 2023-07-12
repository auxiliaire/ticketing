use yew::{classes, html, AttrValue, Children, Component, Properties};

const ICON_CLASS: &str = "fas";

pub struct Field;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub label: AttrValue,
    pub children: Children,
    pub help: Option<AttrValue>,
    pub icon_left: Option<AttrValue>,
    pub icon_right: Option<AttrValue>,
}

pub enum Msg {}

impl Component for Field {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html!(
            <div class="field">
                <label class="label">{ ctx.props().label.as_str() }</label>
                <div class={classes!(self.get_control_classes(ctx))}>
                    { for ctx.props().children.iter() }
                    if ctx.props().icon_left.is_some() {
                        <span class="icon is-small is-left">
                            <i class={classes!(self.get_icon_classes(&ctx.props().icon_left))}></i>
                        </span>
                    }
                    if ctx.props().icon_right.is_some() {
                        <span class="icon is-small is-right">
                            <i class={classes!(self.get_icon_classes(&ctx.props().icon_right))}></i>
                        </span>
                    }
                </div>
                if let Some(help) = &ctx.props().help {
                    <p class="help is-danger">{ help }</p>
                }
              </div>
        )
    }
}

impl Field {
    fn get_control_classes(&self, ctx: &yew::Context<Self>) -> String {
        let mut classes = vec!["control"];
        if let Some(icon_left) = &ctx.props().icon_left {
            classes.push(icon_left.as_str());
        }
        if let Some(icon_right) = &ctx.props().icon_right {
            classes.push(icon_right.as_str());
        }
        classes.join(" ")
    }

    fn get_icon_classes(&self, icon: &Option<AttrValue>) -> String {
        match icon {
            Some(icon_class) => [ICON_CLASS, icon_class.as_str()].join(" "),
            None => String::from(ICON_CLASS),
        }
    }
}
