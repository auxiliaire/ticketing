use super::table_head_sort_order::TableHeadSortOrder;
use implicit_clone::{unsync::IString, ImplicitClone};

#[derive(Clone, Debug, PartialEq)]
pub struct TableHeadSort {
    pub sort: IString,
    pub order: TableHeadSortOrder,
}

impl ImplicitClone for TableHeadSort {}
