use yew::Html;

#[derive(Clone, Default, PartialEq)]
pub struct Dialog {
    pub active: bool,
    pub content: Html,
}
