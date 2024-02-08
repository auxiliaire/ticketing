use crate::helpers::event_helper::get_value_from_input_event;
use implicit_clone::unsync::{IArray, IString};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: AttrValue,
    // TODO: Ideally a map, but that is not quite supported yet.
    pub options: IArray<IString>,
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
    #[prop_or(None)]
    pub placeholder: Option<AttrValue>,
    #[prop_or(true)]
    pub valid: bool,
    #[prop_or(AttrValue::from("select"))]
    pub base_classes: AttrValue,
    #[prop_or(AttrValue::from("is-danger"))]
    pub error_classes: AttrValue,
}

#[function_component(Select)]
pub fn select(props: &Props) -> Html {
    let Props {
        value,
        options,
        on_change,
        placeholder,
        valid,
        base_classes,
        error_classes,
    } = props.clone();

    let get_classes = || match valid {
        true => base_classes.to_string(),
        false => [base_classes.as_str(), error_classes.as_str()].join(" "),
    };

    let onchange =
        move |input_event: Event| on_change.emit(get_value_from_input_event(input_event));

    html! {
        <div class={classes!(get_classes())}>
            <select value={value.clone()} {onchange}>
                if let Some(ph) = placeholder {
                    <option value="" selected={true}>{ph}</option>
                }
                {
                    options.iter().map(|option| {
                        html!{<option selected={value == option}>{ option }</option>}
                    }).collect::<Html>()
                }
            </select>
        </div>
    }
}
