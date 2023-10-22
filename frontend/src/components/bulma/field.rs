use crate::components::component_helper::get_icon_classes;
use implicit_clone::sync::{IArray, IString};
use yew::{classes, html, AttrValue, Children, Component, Html, Properties};

pub struct Field;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub label: Option<AttrValue>,
    pub children: Children,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub has_addons: bool,
    #[prop_or_default]
    pub help: Option<IArray<IString>>,
    #[prop_or_default]
    pub icon_left: Option<AttrValue>,
    #[prop_or_default]
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
            <div class={classes!(self.get_field_classes(ctx))}>
                { self.label(ctx) }
                <div class={classes!(self.get_control_classes(ctx))}>
                    { for ctx.props().children.iter() }
                    if let Some(icon_left) = ctx.props().icon_left.clone() {
                        <span class="icon is-small is-left">
                            <i class={classes!(get_icon_classes(icon_left))}></i>
                        </span>
                    }
                    if let Some(icon_right) = ctx.props().icon_right.clone() {
                        <span class="icon is-small is-right">
                            <i class={classes!(get_icon_classes(icon_right))}></i>
                        </span>
                    }
                </div>
                if let Some(help) = &ctx.props().help {
                    <p class="help is-danger">
                        <ul>
                            {
                                help.iter().map(|message| {
                                    html!{<li>{html! {message}}</li>}
                                }).collect::<Html>()
                            }
                        </ul>
                    </p>
                }
              </div>
        )
    }
}

impl Field {
    fn label(&self, ctx: &yew::Context<Self>) -> Html {
        match &ctx.props().label {
            Some(label) => html! {
                <label class="label">{ label.as_str() }</label>
            },
            None => html! {},
        }
    }

    fn get_field_classes(&self, ctx: &yew::Context<Self>) -> String {
        let mut classes = vec!["field"];
        if let Some(base_classes) = &ctx.props().class {
            let class = base_classes.as_str();
            classes.push(class);
        }
        if ctx.props().has_addons {
            classes.push("has-addons");
        }
        classes.join(" ")
    }

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
}
