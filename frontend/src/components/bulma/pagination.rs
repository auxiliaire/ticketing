use yew::{classes, html, Callback, Component, Context, Properties};

const PAGINATION_LINK_CLASS: &str = "pagination-link";
const PAGINATION_ACTIVE_CLASS: &str = "is-current";
const PAGINATION_DISABLED_CLASS: &str = "is-disabled";
const PAGINATION_PREV_CLASS: &str = "pagination-previous";
const PAGINATION_NEXT_CLASS: &str = "pagination-next";
const MAX_SHOWN_STEPS: u64 = 3;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub total: i64,
    pub offset: u64,
    pub limit: u64,
    pub paginghandler: Callback<u64>, // offset
}

pub enum PaginationMsg {
    ButtonPressed(u64),
}

pub struct Pagination {
    steps: u64,
    current: u64,
}

impl Component for Pagination {
    type Message = PaginationMsg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let steps = Pagination::calculate_steps(ctx.props().total, ctx.props().limit);
        let current = Pagination::calculate_current(ctx.props().offset, ctx.props().limit);
        Self { steps, current }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.steps = Pagination::calculate_steps(ctx.props().total, ctx.props().limit);
        self.current = Pagination::calculate_current(ctx.props().offset, ctx.props().limit);
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PaginationMsg::ButtonPressed(step) => {
                let offset = Pagination::calculate_offset(step, ctx.props().limit);
                if step != self.current {
                    ctx.props().paginghandler.emit(offset);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        match ctx.props().limit == 0 || ctx.props().total == 0 {
            true => html!(),
            false => self.normal_view(ctx),
        }
    }
}

impl Pagination {
    fn normal_view(&self, ctx: &Context<Self>) -> yew::Html {
        let max = std::cmp::min(MAX_SHOWN_STEPS, self.steps);
        let items = (1..max + 1).map(|i| {
            html! (
                <li>
                    <a class={classes!(self.link_classes(i))} aria-label={ format!("Goto page {}", i) } onclick={ctx.link().callback(move |_| PaginationMsg::ButtonPressed(i))}>{ i }</a>
                </li>
            )
        });
        let rest = match self.steps > max {
            true => {
                let i = self.steps;
                let between_step = match i - MAX_SHOWN_STEPS > 2 {
                    true => html!(
                        <li>
                            <span class="pagination-ellipsis">{ "…" }</span>
                        </li>
                    ),
                    false => {
                        let penultimate = i - 1;
                        html!(
                            <li>
                                <a class={classes!(self.link_classes(penultimate))} aria-label={ format!("Goto page {}", penultimate) } onclick={ctx.link().callback(move |_| PaginationMsg::ButtonPressed(penultimate))}>{ penultimate }</a>
                            </li>
                        )
                    }
                };
                html!(
                    <>
                        { between_step }
                        <li>
                            <a class={classes!(self.link_classes(i))} aria-label={ format!("Goto page {}", self.steps) } onclick={ctx.link().callback(move |_| PaginationMsg::ButtonPressed(i))}>{ self.steps }</a>
                        </li>
                    </>
                )
            }
            false => html!(),
        };
        let prev = std::cmp::max(self.current - 1, 1);
        let next = std::cmp::min(self.current + 1, self.steps);
        html!(
            <nav class="pagination is-small" role="navigation" aria-label="pagination">
                <a class={classes!(self.prev_classes())} onclick={ctx.link().callback(move |_| PaginationMsg::ButtonPressed(prev))}>{ "Previous" }</a>
                <a class={classes!(self.next_classes())} onclick={ctx.link().callback(move |_| PaginationMsg::ButtonPressed(next))}>{ "Next page" }</a>
                <ul class="pagination-list">
                    { for items }
                    { rest }
                </ul>
            </nav>
        )
    }

    fn calculate_steps(total: i64, limit: u64) -> u64 {
        (f64::from(total as i32) / f64::from(limit as i32)).ceil() as u64
    }

    fn calculate_current(offset: u64, limit: u64) -> u64 {
        offset / limit + 1
    }

    fn calculate_offset(step: u64, limit: u64) -> u64 {
        (step - 1) * limit
    }

    fn link_classes(&self, i: u64) -> String {
        let mut classes = vec![PAGINATION_LINK_CLASS];
        if self.is_current(i) {
            classes.push(PAGINATION_ACTIVE_CLASS);
        }
        classes.join(" ")
    }

    fn prev_classes(&self) -> String {
        let mut classes = vec![PAGINATION_PREV_CLASS];
        if self.is_first() {
            classes.push(PAGINATION_DISABLED_CLASS);
        }
        classes.join(" ")
    }

    fn next_classes(&self) -> String {
        let mut classes = vec![PAGINATION_NEXT_CLASS];
        if self.is_last() {
            classes.push(PAGINATION_DISABLED_CLASS);
        }
        classes.join(" ")
    }

    fn is_first(&self) -> bool {
        self.current == 1
    }

    fn is_last(&self) -> bool {
        self.current == self.steps
    }

    fn is_current(&self, step: u64) -> bool {
        self.current == step
    }
}
