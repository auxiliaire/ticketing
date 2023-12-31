use super::table_head_sort::TableHeadSort;
use implicit_clone::{unsync::IString, ImplicitClone};

#[derive(Clone, Debug, PartialEq)]
pub struct TableHeadData {
    pub label: IString,
    pub sort: Option<TableHeadSort>,
}

impl ImplicitClone for TableHeadData {}

impl<T> From<T> for TableHeadData
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self {
            label: IString::from(value.to_string()),
            sort: None,
        }
    }
}
