use super::{composite_cell_data::CompositeCellData, table_header::TableHeader};
use implicit_clone::{
    unsync::{IArray, IString},
    ImplicitClone,
};
use shared::dtos::field_index_trait::FieldIndex;
use std::{rc::Rc, str::FromStr};
use yew::{Callback, Html};

pub type HeadProviderFn<F> = Callback<F, Option<TableHeader>>;
pub type CellRendererFn<F, T> = Callback<CompositeCellData<F, T>, Option<Html>>;

#[derive(Debug, PartialEq)]
pub struct TableDataSource<F, T>
where
    F: Clone + FieldIndex + FromStr + ImplicitClone + PartialEq + 'static,
    T: ImplicitClone + PartialEq + 'static,
{
    pub empty_label: IString,
    pub fieldset: IArray<F>,
    pub data: IArray<T>,
    pub has_row_head: bool,
    pub headprovider: Option<HeadProviderFn<F>>,
    pub cellrenderer: CellRendererFn<F, T>,
}

pub type ITableDataSource<F, T> = Rc<TableDataSource<F, T>>;
