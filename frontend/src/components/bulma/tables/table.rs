use super::{
    table_data_source::ITableDataSource, table_head_data::TableHeadData,
    table_head_sort_manager::TableHeadSortManager,
};
use crate::components::bulma::tables::{
    composite_cell_data::CompositeCellData, table_head::TableHead,
};
use implicit_clone::ImplicitClone;
use shared::dtos::getter::Getter;
use std::str::FromStr;
use yew::{classes, html, AttrValue, Callback, Component, Context, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props<F, T, V>
where
    F: Clone + Into<usize> + FromStr + ImplicitClone + PartialEq + ToString + 'static,
    T: Getter<F, V> + ImplicitClone + PartialEq + 'static,
    V: ToString + PartialEq + 'static,
{
    pub datasource: ITableDataSource<F, T, V>,
    #[prop_or_default]
    pub sort: Option<TableHeadData>,
    #[prop_or_default]
    pub sorthandler: Option<Callback<TableHeadData>>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

pub enum TableMsg {
    SortClicked(TableHeadData),
}

pub struct Table<F, T, V>
where
    F: Clone + Into<usize> + FromStr + ImplicitClone + PartialEq + ToString + 'static,
    T: Getter<F, V> + ImplicitClone + PartialEq + 'static,
    V: ToString + PartialEq + 'static,
{
    datasource: ITableDataSource<F, T, V>,
    sortmanager: TableHeadSortManager,
    sorthandler: Option<Callback<TableHeadData>>,
}
impl<F, T, V> Component for Table<F, T, V>
where
    F: Clone + Into<usize> + FromStr + ImplicitClone + PartialEq + ToString + 'static,
    T: Getter<F, V> + ImplicitClone + PartialEq + 'static,
    V: ToString + PartialEq + 'static,
{
    type Message = TableMsg;

    type Properties = Props<F, T, V>;

    fn create(ctx: &Context<Self>) -> Self {
        let mut sortmanager = TableHeadSortManager::from(ctx.props().datasource.fieldset.clone());
        if let Some(data) = ctx.props().sort.clone() {
            sortmanager.update(data);
        }
        Self {
            datasource: ctx.props().datasource.clone(),
            sortmanager,
            sorthandler: ctx.props().sorthandler.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TableMsg::SortClicked(data) => {
                self.sortmanager.update(data.clone());
                if let Some(h) = self.sorthandler.as_ref() {
                    h.emit(data)
                }
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.datasource = ctx.props().datasource.clone();
        if let Some(data) = ctx.props().sort.clone() {
            self.sortmanager.update(data);
        }
        self.sorthandler = ctx.props().sorthandler.clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let header = match self.datasource.has_column_head {
            true => {
                let heads = self.sortmanager.header.iter().map(|data| {
                    html! {
                        <TableHead {data} sorthandler={ctx.link().callback(TableMsg::SortClicked)} />
                    }
                });
                html! {
                    <thead>
                        <tr>
                            { for heads }
                        </tr>
                    </thead>
                }
            }
            false => html!(),
        };

        let rows = self.datasource.data.iter().map(|entry| {
            let cols = self.datasource.fieldset.iter().map(|field| {
                let render = self.datasource.cellrenderer.emit(CompositeCellData {
                    column: field.clone(),
                    data: entry.clone(),
                });
                if let Some(cell) = render {
                    match self.datasource.has_row_head
                        && <F as Into<usize>>::into(field.clone()) == 0usize
                    {
                        true => html! {
                            <th>{ cell }</th>
                        },
                        false => html! {
                            <td>{ cell }</td>
                        },
                    }
                } else {
                    html!()
                }
            });
            html! {
                <tr>{ for cols }</tr>
            }
        });

        match self.datasource.data.is_empty() {
            true => {
                let label = self.datasource.empty_label.clone();
                html! { <em>{ label.as_str() }</em> }
            }
            false => html! {
                <table class={classes!(self.get_table_classes(ctx))}>
                    { header }
                    <tbody>
                        { for rows }
                    </tbody>
                </table>
            },
        }
    }
}

impl<F, T, V> Table<F, T, V>
where
    F: Clone + Into<usize> + FromStr + ImplicitClone + PartialEq + ToString + 'static,
    T: Getter<F, V> + ImplicitClone + PartialEq + 'static,
    V: ToString + PartialEq + 'static,
{
    fn get_table_classes(&self, ctx: &Context<Self>) -> String {
        let mut classes = vec!["table", "is-fullwidth", "is-hoverable"];
        if let Some(base_classes) = &ctx.props().class {
            let class = base_classes.as_str();
            classes.push(class);
        }
        classes.join(" ")
    }
}
