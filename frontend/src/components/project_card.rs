use crate::pages::project_page::ProjectPageMsg;
use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub name: AttrValue,
    pub id: Option<u64>,
    // pub on_change: Callback<u64>,
}

pub struct ProjectCard {
    name: AttrValue,
    id: Option<u64>,
}

impl Component for ProjectCard {
    type Message = ProjectPageMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            name: ctx.props().name.clone(),
            id: ctx.props().id,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if let Some(_id) = ctx.props().id {
            todo!("Check whether this fetch is really needed");
            //ProjectService::fetch(
            //    token,
            //    id,
            //    ctx.link().callback(ProjectPageMsg::FetchedProject),
            //);
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { name, id, .. } = self;
        html! {
            <div class="card">
                <div class="card-content">
                    <div class="media">
                        <div class="media-content">
                            <p class="title is-3">{ name }</p>
                        </div>
                    </div>
                </div>
                <footer class="card-footer">
                    if let Some(id) = id {
                        <Link<Route> classes={classes!("card-footer-item")} to={Route::Project { id: *id }}>
                            { "Details" }
                        </Link<Route>>
                    }
                </footer>
            </div>
        }
    }
}
