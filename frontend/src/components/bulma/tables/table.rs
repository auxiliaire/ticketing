use super::table_data_source::ITableDataSource;
use crate::components::bulma::tables::{
    table_cell_renderer_trait::TableCellRenderer, table_header_renderer::TableHeaderRenderer, composite_cell_data::CompositeCellData,
};
use implicit_clone::ImplicitClone;
use shared::dtos::field_index_trait::FieldIndex;
use std::str::FromStr;
use yew::{classes, html, AttrValue, Component, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props<F, T>
where
    F: Clone + FieldIndex + FromStr + ImplicitClone + PartialEq + 'static,
    T: ImplicitClone + PartialEq + 'static,
{
    pub datasource: ITableDataSource<F, T>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

pub enum Msg {}

pub struct Table<F, T>
where
    F: Clone + FieldIndex + FromStr + ImplicitClone + PartialEq + 'static,
    T: ImplicitClone + PartialEq + 'static,
{
    datasource: ITableDataSource<F, T>,
}
impl<F, T> Component for Table<F, T>
where
    F: Clone + FieldIndex + FromStr + ImplicitClone + PartialEq + 'static,
    T: ImplicitClone + PartialEq + 'static,
{
    type Message = Msg;

    type Properties = Props<F, T>;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            datasource: ctx.props().datasource.clone(),
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        self.datasource = ctx.props().datasource.clone();
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let head = match self.datasource.headprovider.clone() {
            Some(headprovider) => {
                let headers = self
                    .datasource
                    .fieldset
                    .iter()
                    .filter_map(|field| TableHeaderRenderer::render(headprovider.emit(field)));
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
                let render = self
                    .datasource
                    .cellrenderer
                    .emit(CompositeCellData { column: field.clone(), data: entry.clone() });
                if let Some(cell) = render {
                    match self.datasource.has_row_head && field.index() == 0 {
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

impl<F, T> Table<F, T>
where
    F: Clone + FieldIndex + FromStr + ImplicitClone + PartialEq + 'static,
    T: ImplicitClone + PartialEq + 'static,
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
