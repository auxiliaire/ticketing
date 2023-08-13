use super::table_data_source::SortHandler;
use yew::Html;

pub trait TableCellRenderer<T> {
    fn render(cell_data: T, sorthandler: Option<SortHandler>) -> Option<Html>;
}
