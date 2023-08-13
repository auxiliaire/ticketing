use crate::components::bulma::tables::{
    composite_cell_data::CompositeCellData, table_data_source::CellRendererFn,
};
use implicit_clone::ImplicitClone;
use shared::dtos::getter::Getter;
use std::str::FromStr;
use yew::{html, Callback};

pub struct SimpleCellRenderer {}

impl SimpleCellRenderer {
    pub fn create<F, T, V>() -> CellRendererFn<F, T>
    where
        F: Clone + Into<usize> + FromStr + ImplicitClone + PartialEq + 'static,
        T: Getter<F, V> + ImplicitClone + PartialEq + 'static,
        V: ToString + PartialEq + 'static,
    {
        Callback::from(|c: CompositeCellData<F, T>| {
            let data = c.data;
            let field = c.column;
            let value = data.get(field);
            Some(html!({ value }))
        })
    }
}
