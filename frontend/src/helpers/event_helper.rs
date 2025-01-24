use implicit_clone::sync::IArray;
use std::sync::Arc;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{DragEvent, Event, File, HtmlInputElement, HtmlSelectElement};
use yew::AttrValue;

pub fn get_file_from_change_event(e: Event) -> Option<File> {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    if let Some(target) = event_target.dyn_ref::<HtmlInputElement>() {
        return target.files().and_then(|files| files.get(0));
    }
    None
}

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

pub fn get_values_from_select_change<K>(e: Event) -> IArray<K>
where
    K: std::str::FromStr + implicit_clone::ImplicitClone,
{
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let mut selected: Vec<K> = vec![];
    if let Some(target) = event_target.dyn_ref::<HtmlSelectElement>() {
        let selected_options = target.selected_options();
        for i in 0..selected_options.length() {
            let element = selected_options.item(i).unwrap_throw();
            let value_string = element.get_attribute("value").unwrap_throw();
            if let Ok(value) = value_string.as_str().parse::<K>() {
                selected.push(value);
            }
        }
    }
    IArray::Rc(Arc::from(selected.as_slice()))
}

pub fn set_transfer_data(e: DragEvent, data: &str) -> Result<(), JsValue> {
    let event_data_transfer = e.data_transfer();
    match event_data_transfer {
        Some(data_transfer) => data_transfer.set_data("text", data),
        None => Err(JsValue::undefined()),
    }
}

pub fn get_transfer_data(e: DragEvent) -> Result<String, JsValue> {
    let event_data_transfer = e.data_transfer();
    match event_data_transfer {
        Some(data_transfer) => data_transfer.get_data("text"),
        None => Err(JsValue::undefined()),
    }
}
