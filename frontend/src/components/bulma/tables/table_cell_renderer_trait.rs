use yew::Html;

pub trait TableCellRenderer<T> {
    fn render(cell_data: T) -> Option<Html>;
}
