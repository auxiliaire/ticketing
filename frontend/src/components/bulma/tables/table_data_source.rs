use super::{
    cell_renderers::simple_cell_renderer::SimpleCellRenderer,
    composite_cell_data::CompositeCellData, table_head_data::TableHeadData,
};
use implicit_clone::{
    unsync::{IArray, IString},
    ImplicitClone,
};
use shared::dtos::getter::Getter;
use std::{marker::PhantomData, rc::Rc, str::FromStr};
use strum::IntoEnumIterator;
use yew::{Callback, Html};

pub type CellRendererFn<F, T> = Callback<CompositeCellData<F, T>, Option<Html>>;
pub type SortHandler = Callback<TableHeadData>;

#[derive(Debug, PartialEq)]
pub struct TableDataSource<F, T, V>
where
    F: Clone + Into<usize> + FromStr + ImplicitClone + PartialEq + 'static,
    T: Getter<F, V> + ImplicitClone + PartialEq + 'static,
    V: ToString + PartialEq + 'static,
{
    pub empty_label: IString,
    pub fieldset: IArray<F>,
    pub data: IArray<T>,
    pub has_column_head: bool,
    pub has_row_head: bool,
    pub cellrenderer: CellRendererFn<F, T>,
    pub phantom: PhantomData<V>,
}

pub type ITableDataSource<F, T, V> = Rc<TableDataSource<F, T, V>>;

impl<F, T, V> Default for TableDataSource<F, T, V>
where
    F: Clone + Into<usize> + FromStr + ImplicitClone + IntoEnumIterator + PartialEq + 'static,
    T: Getter<F, V> + ImplicitClone + PartialEq + 'static,
    V: ToString + PartialEq + 'static,
{
    fn default() -> Self {
        Self {
            empty_label: IString::from("No entries"),
            fieldset: IArray::from(F::iter().collect::<Vec<F>>()),
            data: Default::default(),
            has_column_head: Default::default(),
            has_row_head: Default::default(),
            cellrenderer: SimpleCellRenderer::create(),
            phantom: PhantomData,
        }
    }
}
