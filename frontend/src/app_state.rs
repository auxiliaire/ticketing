use crate::dialog::Dialog;
use std::rc::Rc;
use yew::Callback;

#[derive(Clone, Default, PartialEq)]
pub struct AppState {
    pub update_dialog: Callback<Rc<Dialog>>,
    pub close_dialog: Callback<()>,
    pub navbar_active: bool,
}
