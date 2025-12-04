use crate::components::consts::ICON_CLASS;
use implicit_clone::unsync::IString;
use yew::{html, AttrValue, Html};

pub fn get_icon_classes(icon: AttrValue) -> String {
    [ICON_CLASS, icon.as_str()].join(" ").trim().to_owned()
}

pub fn to_html(ustring: IString) -> Html {
    html! {
        ustring.clone().to_string()
    }
}
