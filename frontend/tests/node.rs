use frontend::components::html::checkbox::{Checkbox, Props};
use wasm_bindgen_test::*;
use yew::prelude::*;

#[macro_use]
mod common;

#[wasm_bindgen_test]
fn pass() {
    let props = Props {
        checked: false,
        on_change: Callback::default(),
        valid: true,
        base_classes: AttrValue::from(""),
        error_classes: AttrValue::from("is-danger"),
    };

    assert_expected_output!(Checkbox, props, r#"<div class="container"></div>"#);
}
