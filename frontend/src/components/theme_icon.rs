use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub dark: bool,
}

#[function_component]
pub fn ThemeIcon(props: &Props) -> Html {
    let icon_class = match props.dark {
        true => "fa-regular fa-sun",
        false => "fa-solid fa-moon",
    };
    html! {
        <i class={classes!("fas", icon_class)}></i>
    }
}
