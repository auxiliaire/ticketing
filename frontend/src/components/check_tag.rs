use yew::{classes, component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub checked: bool,
}

#[component]
pub fn CheckTag(props: &Props) -> Html {
    let icon_class = match props.checked {
        true => "fa-check",
        false => "fa-times",
    };
    html! {
        <span class="button is-small is-static">
            <span class="icon is-small">
                <i class={classes!("fas", icon_class)}></i>
            </span>
        </span>
    }
}
