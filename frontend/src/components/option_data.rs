use yew::AttrValue;

pub trait OptionData {
    fn get_key(&self) -> AttrValue;
    fn get_label(&self) -> AttrValue;
}
