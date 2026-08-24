use yew::{classes, component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub dark: bool,
}

#[component]
pub fn ThemeIcon(props: &Props) -> Html {
    let icon_class = match props.dark {
        true => "fa-regular fa-sun",
        false => "fa-solid fa-moon",
    };
    html! {
        <i class={classes!("fas", icon_class)}></i>
    }
}
