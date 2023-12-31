use crate::pages::user_page::Msg;
use crate::services::user_service::UserService;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub name: AttrValue,
    pub id: Option<u64>,
}

pub struct UserCard {
    name: AttrValue,
    id: Option<u64>,
}
impl Component for UserCard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            name: ctx.props().name.clone(),
            id: ctx.props().id,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if let Some(id) = ctx.props().id {
            UserService::fetch(id, ctx.link().callback(Msg::FetchedUser));
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { name, id } = self;
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
                        <Link<Route> classes={classes!("card-footer-item")} to={Route::User { id: *id }}>
                            { "Profile" }
                        </Link<Route>>
                    }
                </footer>
            </div>
        }
    }
}
