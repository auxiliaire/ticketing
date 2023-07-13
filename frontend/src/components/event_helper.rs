use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, HtmlSelectElement};
use yew::AttrValue;

pub fn get_value_from_input_event(e: Event) -> AttrValue {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    if let Some(target) = event_target.dyn_ref::<HtmlSelectElement>() {
        return target.value().into();
    }
    if let Some(target) = event_target.dyn_ref::<HtmlInputElement>() {
        return target.value().into();
    }
    AttrValue::from("")
}
