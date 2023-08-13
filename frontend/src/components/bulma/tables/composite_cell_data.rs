use implicit_clone::ImplicitClone;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeCellData<F, T>
where
    F: Clone + Into<usize> + FromStr + ImplicitClone + PartialEq + 'static,
    T: ImplicitClone + PartialEq + 'static,
{
    pub column: F,
    pub data: T,
}
