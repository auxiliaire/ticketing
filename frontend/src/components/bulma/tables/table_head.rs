use super::{
    table_head_data::TableHeadData, table_head_sort::TableHeadSort,
    table_head_sort_order::TableHeadSortOrder,
};
use implicit_clone::unsync::IString;
use yew::{classes, html, Callback, Component, Context, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub data: TableHeadData,
    #[prop_or_default]
    pub sorthandler: Option<Callback<TableHeadData>>,
}

pub enum TableHeadMsg {
    SortClicked(),
}

pub struct TableHead {
    data: TableHeadData,
    sorthandler: Option<Callback<TableHeadData>>,
}

impl Component for TableHead {
    type Message = TableHeadMsg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            data: ctx.props().data.clone(),
            sorthandler: ctx.props().sorthandler.clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.data = ctx.props().data.clone();
        self.sorthandler = ctx.props().sorthandler.clone();
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TableHeadMsg::SortClicked() => {
                if let Some(mut sort) = self.data.sort.clone() {
                    sort.order = match sort.order {
                        TableHeadSortOrder::Asc => TableHeadSortOrder::Desc,
                        TableHeadSortOrder::Desc => TableHeadSortOrder::Asc,
                    };
                    self.data.sort = Some(sort);
                } else {
                    self.data.sort = Some(TableHeadSort {
                        sort: IString::from(self.data.label.clone().to_lowercase()),
                        order: TableHeadSortOrder::Asc,
                    });
                }
                let payload = self.data.clone();
                if let Some(h) = self.sorthandler.as_ref() {
                    h.emit(payload)
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        html! {
            <th>{ self.head_view(ctx) }</th>
        }
    }
}

impl TableHead {
    fn head_view(&self, ctx: &Context<Self>) -> Html {
        let label = self.data.label.clone();
        match self.sorthandler {
            Some(_) => {
                let on_click = |_| TableHeadMsg::SortClicked();
                html! {
                    <a onclick={ctx.link().callback(on_click)}>{ self.label_view() }</a>
                }
            }
            None => html! { label },
        }
    }

    fn label_view(&self) -> Html {
        let label = self.data.label.clone();
        match self.data.sort.clone() {
            Some(TableHeadSort { sort: _, order }) => html! {
                <span class="icon-text">
                    <span>{ label.as_str() }</span>
                    <span class="icon">{ self.icon_view(order) }</span>
                </span>
            },
            None => html! { label },
        }
    }

    fn icon_view(&self, order: TableHeadSortOrder) -> Html {
        let icon = match order {
            TableHeadSortOrder::Asc => "fa-sort-up",
            TableHeadSortOrder::Desc => "fa-sort-down",
        };
        html! {
            <i class={classes!("fa-solid", icon)}></i>
        }
    }
}
