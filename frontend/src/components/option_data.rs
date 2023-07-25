use implicit_clone::unsync::IString;

pub trait OptionData {
    fn get_key(&self) -> IString;
    fn get_label(&self) -> IString;
}
