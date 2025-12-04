use super::table_head_data::TableHeadData;
use implicit_clone::{unsync::IArray, unsync::IString, ImplicitClone};
use std::str::FromStr;

pub struct TableHeadSortManager {
    pub header: IArray<TableHeadData>,
}

impl TableHeadSortManager {
    pub fn update(&mut self, data: TableHeadData) {
        let new_header = self
            .header
            .iter()
            .map(|old_data| match old_data.label == data.label {
                true => data.clone(),
                false => TableHeadData {
                    label: old_data.label.clone(),
                    sort: None,
                },
            })
            .collect();
        self.header = new_header;
    }
}

impl<F> From<IArray<F>> for TableHeadSortManager
where
    F: Clone + Into<usize> + FromStr + ImplicitClone + PartialEq + ToString + 'static,
{
    fn from(value: IArray<F>) -> Self {
        let header = value
            .iter()
            .map(|f| TableHeadData {
                label: IString::from(f.to_string()),
                sort: None,
            })
            .collect();
        Self { header }
    }
}
