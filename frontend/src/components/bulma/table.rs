use implicit_clone::{
    unsync::{self, IArray, IString},
    ImplicitClone,
};
use shared::dtos::field_index_trait::FieldIndex;
use std::{rc::Rc, str::FromStr};
use strum::{Display, EnumString};
use yew::{classes, html, AttrValue, Callback, Component, Html, Properties};

pub trait TableCellRenderer<T> {
    fn render(column: T) -> Html;
}

pub struct TableHeaderRenderer {}
impl TableCellRenderer<Option<TableHeader>> for TableHeaderRenderer {
    fn render(header: Option<TableHeader>) -> Html {
        match header {
            Some(column) => html! {
                <th>{ column.label }</th>
            },
            None => html!(),
        }
    }
}

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
pub enum TableHeadSortOrder {
    #[strum(serialize = "asc")]
    Asc,
    #[strum(serialize = "desc")]
    Desc,
}

impl ImplicitClone for TableHeadSortOrder {}

#[derive(Clone, Debug, PartialEq)]
pub struct TableHeadSort {
    pub sort: IString,
    pub order: TableHeadSortOrder,
}

impl ImplicitClone for TableHeadSort {}

#[derive(Clone, Debug, PartialEq)]
pub struct TableHeader {
    pub label: IString,
    pub sort: Option<TableHeadSort>,
}

impl ImplicitClone for TableHeader {}

#[derive(Debug, PartialEq)]
pub struct TableDataSource<F, T>
where
    F: Clone + FieldIndex + FromStr + ImplicitClone + PartialEq + 'static,
    T: ImplicitClone + PartialEq + 'static,
{
    pub empty_label: unsync::IString,
    pub fieldset: IArray<F>,
    pub data: IArray<T>,
    pub has_row_head: bool,
    pub headprovider: Option<Callback<F, Option<TableHeader>>>,
    pub cellrenderer: Callback<(F, T), Option<Html>>,
}

impl<F, T> Default for TableDataSource<F, T>
where
    F: Clone + FieldIndex + FromStr + ImplicitClone + PartialEq + 'static,
    T: ImplicitClone + PartialEq + 'static,
{
    fn default() -> Self {
        Self {
            empty_label: unsync::IString::from("No entries"),
            fieldset: Default::default(),
            data: Default::default(),
            has_row_head: Default::default(),
            headprovider: Default::default(),
            cellrenderer: Callback::from(|_| None),
        }
    }
}

pub type ITableDataSource<F, T> = Rc<TableDataSource<F, T>>;

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
                    .map(|field| TableHeaderRenderer::render(headprovider.emit(field)));
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
                    .emit((field.clone(), entry.clone()));
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
