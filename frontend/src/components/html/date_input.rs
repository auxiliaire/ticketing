use yew::prelude::*;

use crate::components::event_helper::get_value_from_input_event;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: AttrValue,
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or(AttrValue::from("Type here"))]
    pub placeholder: AttrValue,
    #[prop_or(true)]
    pub valid: bool,
    #[prop_or(AttrValue::from("input"))]
    pub base_classes: AttrValue,
    #[prop_or(AttrValue::from("is-danger"))]
    pub error_classes: AttrValue,
}

#[function_component(DateInput)]
pub fn text_input(props: &Props) -> Html {
    let Props {
        value,
        on_change,
        on_click,
        placeholder,
        valid,
        base_classes,
        error_classes,
    } = props.clone();

    let get_classes = || match valid {
        true => base_classes.to_string(),
        false => [base_classes.as_str(), error_classes.as_str()].join(" "),
    };

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event.into()))
    });

    let onclick = Callback::from(move |mouse_event| {
        on_click.emit(mouse_event);
    });

    html! {
        <input class={classes!(get_classes())} type="date" {value} {oninput} {placeholder} {onclick} />
    }
}
