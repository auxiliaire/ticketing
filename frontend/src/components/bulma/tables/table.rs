use super::{table_data_source::ITableDataSource, table_header::TableHeader};
use crate::components::bulma::tables::{
    composite_cell_data::CompositeCellData, table_cell_renderer_trait::TableCellRenderer,
    table_header_renderer::TableHeaderRenderer,
};
use implicit_clone::ImplicitClone;
use shared::dtos::getter::Getter;
use std::str::FromStr;
use yew::{classes, html, AttrValue, Callback, Component, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props<F, T, V>
where
    F: Clone + Into<usize> + FromStr + ImplicitClone + PartialEq + 'static,
    T: Getter<F, V> + ImplicitClone + PartialEq + 'static,
    V: ToString + PartialEq + 'static,
{
    pub datasource: ITableDataSource<F, T, V>,
    #[prop_or_default]
    pub sorthandler: Option<Callback<TableHeader>>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

pub enum Msg {}

pub struct Table<F, T, V>
where
    F: Clone + Into<usize> + FromStr + ImplicitClone + PartialEq + 'static,
    T: Getter<F, V> + ImplicitClone + PartialEq + 'static,
    V: ToString + PartialEq + 'static,
{
    datasource: ITableDataSource<F, T, V>,
    sorthandler: Option<Callback<TableHeader>>,
}
impl<F, T, V> Component for Table<F, T, V>
where
    F: Clone + Into<usize> + FromStr + ImplicitClone + PartialEq + 'static,
    T: Getter<F, V> + ImplicitClone + PartialEq + 'static,
    V: ToString + PartialEq + 'static,
{
    type Message = Msg;

    type Properties = Props<F, T, V>;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            datasource: ctx.props().datasource.clone(),
            sorthandler: ctx.props().sorthandler.clone(),
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        self.datasource = ctx.props().datasource.clone();
        self.sorthandler = ctx.props().sorthandler.clone();
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let head = match self.datasource.headprovider.clone() {
            Some(headprovider) => {
                let headers = self.datasource.fieldset.iter().filter_map(|field| {
                    TableHeaderRenderer::render(headprovider.emit(field), self.sorthandler.clone())
                });
                html! {
                    <thead>
                        <tr>
                            { for headers }
                        </tr>
                    </thead>
                }
            }
            None => html!(),
        };

        let rows = self.datasource.data.iter().map(|entry| {
            let cols = self.datasource.fieldset.iter().map(|field| {
                let render = self.datasource.cellrenderer.emit(CompositeCellData {
                    column: field.clone(),
                    data: entry.clone(),
                });
                if let Some(cell) = render {
                    match self.datasource.has_row_head && field.into() == 0 {
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
            true => html! { <em>{ self.datasource.empty_label.clone() }</em> },
            false => html! {
                <table class={classes!(self.get_table_classes(ctx))}>
                    { head }
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
    F: Clone + Into<usize> + FromStr + ImplicitClone + PartialEq + 'static,
    T: Getter<F, V> + ImplicitClone + PartialEq + 'static,
    V: ToString + PartialEq + 'static,
{
    fn get_table_classes(&self, ctx: &yew::Context<Self>) -> String {
        let mut classes = vec!["table", "is-fullwidth", "is-hoverable"];
        if let Some(base_classes) = &ctx.props().class {
            let class = base_classes.as_str();
            classes.push(class);
        }
        classes.join(" ")
    }
}
