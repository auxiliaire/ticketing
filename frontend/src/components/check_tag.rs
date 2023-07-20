use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub checked: bool,
}

#[function_component]
pub fn CheckTag(props: &Props) -> Html {
    match props.checked {
        true => {
            html! {
                <span class="button is-small is-static">
                    <span class="icon is-small">
                        <i class="fas fa-check"></i>
                    </span>
                </span>
            }
        }
        false => {
            html! {
                <span class="button is-small is-static">
                    <span class="icon is-small">
                        <i class="fas fa-times"></i>
                    </span>
                </span>
            }
        }
    }
}
