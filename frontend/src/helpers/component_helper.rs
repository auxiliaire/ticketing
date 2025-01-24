use crate::components::consts::ICON_CLASS;
use implicit_clone::unsync::IString;

pub fn get_icon_classes(icon: IString) -> String {
    [ICON_CLASS, icon.as_str()].join(" ").trim().to_owned()
}
