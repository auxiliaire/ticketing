use super::{
    table_cell_renderer_trait::TableCellRenderer, table_data_source::SortHandler,
    table_head::TableHead, table_head_data::TableHeadData,
};
use yew::{html, Html};

#[derive(Debug)]
pub struct TableHeadRenderer {}

impl TableCellRenderer<Option<TableHeadData>> for TableHeadRenderer {
    fn render(header: Option<TableHeadData>, sorthandler: Option<SortHandler>) -> Option<Html> {
        header.map(|data| {
            html! {
                <TableHead {data} {sorthandler} />
            }
        })
    }
}
