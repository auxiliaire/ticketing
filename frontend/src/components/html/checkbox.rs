use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub checked: bool,
    #[prop_or_default]
    pub on_change: Callback<bool>,
    #[prop_or(true)]
    pub valid: bool,
    #[prop_or(AttrValue::from(""))]
    pub base_classes: AttrValue,
    #[prop_or(AttrValue::from("is-danger"))]
    pub error_classes: AttrValue,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let Props {
        checked,
        on_change,
        valid,
        base_classes,
        error_classes,
    } = props.clone();
    let value = use_state_eq(|| checked);

    let get_classes = || match valid {
        true => base_classes.to_string(),
        false => [base_classes.as_str(), error_classes.as_str()].join(" "),
    };

    let onchange = Callback::from(move |_| {
        let value = value.clone();
        let new_value = !*value;
        value.set(new_value);
        on_change.emit(new_value);
    });

    html! {
        <input class={classes!(get_classes())} type="checkbox" {checked} {onchange} />
    }
}
