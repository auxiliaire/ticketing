use yew::Html;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Dialog {
    pub active: bool,
    pub content: Html,
}
