use super::{table_cell_renderer_trait::TableCellRenderer, table_header::TableHeader};
use yew::{html, Html};

#[derive(Debug)]
pub struct TableHeaderRenderer {}

impl TableCellRenderer<Option<TableHeader>> for TableHeaderRenderer {
    fn render(header: Option<TableHeader>) -> Option<Html> {
        match header {
            Some(column) => Some(html! {
                <th>{ column.label }</th>
            }),
            None => None,
        }
    }
}
