use super::{
    table_cell_renderer_trait::TableCellRenderer, table_data_source::SortHandler,
    table_header::TableHeader,
};
use yew::{html, Callback, Html};

#[derive(Debug)]
pub struct TableHeaderRenderer {}

impl TableCellRenderer<Option<TableHeader>> for TableHeaderRenderer {
    fn render(header: Option<TableHeader>, sorthandler: Option<SortHandler>) -> Option<Html> {
        let label_view =
            |column: TableHeader, sorthandler: Option<Callback<TableHeader>>| match sorthandler {
                Some(handler) => {
                    let label = column.label.clone();
                    html! {
                        <a onclick={move |_| handler.emit(column.clone())}>{ label }</a>
                    }
                }
                None => html! { column.label },
            };
        header.map(|column| {
            html! {
                <th>{ label_view(column, sorthandler) }</th>
            }
        })
    }
}
