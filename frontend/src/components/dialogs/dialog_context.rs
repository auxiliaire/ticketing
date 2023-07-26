use yew::Callback;

#[derive(Clone, PartialEq)]
pub struct DialogContext {
    pub closehandler: Callback<()>,
}
