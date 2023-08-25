use yew::{html, Component, Properties};

pub struct Pagination {
    steps: u64,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub total: i64,
    pub offset: u64,
    pub limit: u64,
}

pub enum PaginationMsg {}

impl Component for Pagination {
    type Message = PaginationMsg;

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let steps = (f64::from(ctx.props().total as i32) / f64::from(ctx.props().limit as i32))
            .ceil() as u64;
        Self { steps }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        let max = std::cmp::min(3, self.steps);
        log::debug!("steps: {}", self.steps);
        log::debug!("max: {}", max);
        let items = (1..max + 1).map(|i| {
            html! (
                <li>
                    <a class="pagination-link" aria-label={ format!("Goto page {}", i) }>{ i }</a>
                </li>
            )
        });
        let rest = match self.steps > max {
            true => html!(
                <>
                    <li>
                        <span class="pagination-ellipsis">{ "…" }</span>
                    </li>
                    <li>
                        <a class="pagination-link" aria-label={ format!("Goto page {}", self.steps) }>{ self.steps }</a>
                    </li>
                </>
            ),
            false => html!(),
        };
        html!(
            <nav class="pagination" role="navigation" aria-label="pagination">
                <a class="pagination-previous">{ "Previous" }</a>
                <a class="pagination-next">{ "Next page" }</a>
                <ul class="pagination-list">
                    { for items }
                    { rest }
                </ul>
            </nav>
        )
    }
}
