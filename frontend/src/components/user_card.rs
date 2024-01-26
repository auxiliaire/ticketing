use crate::pages::user_page::UserPageMsg;
use crate::route::Route;
use crate::services::user_service::UserService;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub name: AttrValue,
    pub username: AttrValue,
    pub id: Option<Uuid>,
}

pub struct UserCard {
    name: AttrValue,
    username: AttrValue,
    id: Option<Uuid>,
}
impl Component for UserCard {
    type Message = UserPageMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            name: ctx.props().name.clone(),
            username: ctx.props().username.clone(),
            id: ctx.props().id,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if let Some(id) = ctx.props().id {
            todo!("Check whether this fetch is really needed");
            // UserService::fetch(id, ctx.link().callback(UserPageMsg::FetchedUser));
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { name, username, id } = self;
        html! {
            <div class="card">
                <div class="card-content">
                    <div class="media">
                        <div class="media-content">
                            <p class="title is-3">{ name }</p>
                            <p class="subtitle is-6">{ username }</p>
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
