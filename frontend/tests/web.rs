use frontend::components::html::checkbox::{Checkbox, Props};
use gloo_utils::document;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use yew::prelude::*;

#[macro_use]
mod common;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Clone, Debug, PartialEq)]
struct Context {
    checked: bool,
}

#[function_component]
fn App() -> Html {
    let checked = use_state(|| false);
    let on_change = {
        let checked = checked.clone();
        Callback::from(move |_| checked.set(!*checked))
    };
    html! {
        <div>
            <Checkbox checked={*checked} {on_change}/>
        </div>
    }
}

#[wasm_bindgen_test]
fn test_app() {
    yew::Renderer::<App>::new().render();

    let inputs = document().get_elements_by_tag_name("input");

    assert_eq!(inputs.length(), 1, "An input tag should be rendered.");
}

#[wasm_bindgen_test]
fn test_component() {
    let props = Props {
        checked: false,
        on_change: Callback::default(),
        valid: true,
        base_classes: AttrValue::from(""),
        error_classes: AttrValue::from("is-danger"),
    };

    assert_expected_output!(Checkbox, props, r#"<div class="container"></div>"#);
}
