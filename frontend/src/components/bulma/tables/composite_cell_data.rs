use std::str::FromStr;

use implicit_clone::ImplicitClone;
use shared::dtos::field_index_trait::FieldIndex;

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeCellData<F, T>
where
    F: Clone + FieldIndex + FromStr + ImplicitClone + PartialEq + 'static,
    T: ImplicitClone + PartialEq + 'static,
{
    pub column: F,
    pub data: T,
}
