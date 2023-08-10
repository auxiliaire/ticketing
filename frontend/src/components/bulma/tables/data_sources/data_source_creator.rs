use crate::components::bulma::tables::table_data_source::ITableDataSource;
use implicit_clone::ImplicitClone;
use shared::dtos::field_index_trait::FieldIndex;
use std::str::FromStr;

pub trait DataSourceCreator<S, F, T>
where
    F: Clone + FieldIndex + FromStr + ImplicitClone + PartialEq + 'static,
    T: ImplicitClone + PartialEq + 'static,
{
    fn create(source: S) -> ITableDataSource<F, T>;
}
