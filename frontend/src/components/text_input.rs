use yew::prelude::*;

use crate::components::event_helper::get_value_from_input_event;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: AttrValue,
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
    #[prop_or(AttrValue::from("Type here"))]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub mask: bool,
    #[prop_or(true)]
    pub valid: bool,
    #[prop_or(String::from("input"))]
    pub base_classes: String,
    #[prop_or(String::from("is-danger"))]
    pub error_classes: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props {
        value,
        on_change,
        placeholder,
        mask,
        valid,
        base_classes,
        error_classes,
    } = props.clone();

    let get_classes = || match valid {
        true => base_classes,
        false => [base_classes.as_str(), error_classes.as_str()].join(" "),
    };

    let get_type = || match mask {
        true => "password",
        false => "text",
    };

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event))
    });

    html! {
        <input class={classes!(get_classes())} type={get_type()} {value} {oninput} {placeholder} />
    }
}
